mod ai;
mod physis;

use ai::provider::ProviderCascade;
use ai::agent::{run_agent, AgentConfig};
use ai::tools::ToolRegistry;
use physis::OntologicalMap;

use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use axum::{extract::State, routing::get, Json, Router};

use dotenvy::dotenv;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct AppState {
    map: OntologicalMap,
    transcripts: Vec<String>,
}

type SharedState = Arc<Mutex<AppState>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let shared_state: SharedState = Arc::new(Mutex::new(AppState::default()));

    // Start Web Server
    let app_state_clone = shared_state.clone();
    tokio::spawn(async move {
        let app = Router::new()
            .route("/api/data", get(get_data))
            .with_state(app_state_clone);

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        println!("Visualization UI available at http://localhost:3000");
        println!("(Note: Since this is a demo, we serve a static HTML string at the root)");
        
        let root_app = app.route("/", get(index));
        axum::serve(listener, root_app).await.unwrap();
    });

    println!("Physis Ontological Mapper starting...");
    println!("Offline Mode: Will attempt local Ollama (llama3.2) for extraction.");
    
    let cascade = ProviderCascade::from_env();
    let tools = ToolRegistry::new();

    let recording_duration_secs = 15;

    loop {
        println!("\n--- Recording {}s chunk ---", recording_duration_secs);
        let wav_path = "chunk.wav";

        let status = Command::new("arecord")
            .arg("-d")
            .arg(recording_duration_secs.to_string())
            .arg("-f")
            .arg("S16_LE")
            .arg("-r")
            .arg("16000")
            .arg("-t")
            .arg("wav")
            .arg(wav_path)
            .status()?;

        if !status.success() {
            eprintln!("arecord failed");
            tokio::time::sleep(Duration::from_secs(2)).await;
            continue;
        }

        println!("Transcribing...");
        // Using Groq/OpenRouter for transcription as local Whisper is heavy, 
        // but extraction will be local.
        let transcript = match cascade.transcribe(wav_path).await {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Transcription error: {}", e);
                continue;
            }
        };

        if transcript.trim().is_empty() {
            continue;
        }

        {
            let mut state = shared_state.lock().unwrap();
            state.transcripts.push(transcript.clone());
        }

        println!("Transcript: \"{}\"", transcript);

        println!("Extracting ontology (local AI)...");
        let ontologist_config = AgentConfig {
            system_prompt: r#"You are Physis. Extract entities and relationships. 
Output ONLY valid JSON:
{
  "entities": {
    "id": { "id": "id", "name": "Name", "kind": "kind", "description": "desc", "attributes": {} }
  },
  "relationships": [
    { "source": "id1", "target": "id2", "predicate": "verb", "weight": 1.0 }
  ]
}"# .into(),
            max_tokens: 1024,
            detail: "low".into(),
        };

        // We use "ollama" explicitly if it exists in cascade, otherwise fall back
        let result = run_agent(
            &cascade,
            &tools,
            &ontologist_config,
            &[],
            &format!("Update map: {}", transcript),
            None,
            "DATA" // Agent will try to route this to Ollama if configured
        ).await;

        match result {
            Ok(output) => {
                if let (Some(start), Some(end)) = (output.text.find('{'), output.text.rfind('}')) {
                    if let Ok(new_map) = serde_json::from_str::<OntologicalMap>(&output.text[start..=end]) {
                        let mut state = shared_state.lock().unwrap();
                        state.map.merge(new_map);
                        println!("Map updated. Entities: {}", state.map.entities.len());
                    }
                }
            }
            Err(e) => eprintln!("Extraction error: {}", e),
        }
    }
}

async fn get_data(State(state): State<SharedState>) -> Json<AppState> {
    let data = state.lock().unwrap().clone();
    Json(data)
}

async fn index() -> axum::response::Html<&'static str> {
    axum::response::Html(include_str!("index.html"))
}

#[derive(Serialize, Deserialize)]
struct TranscriptUpdate {
    text: String,
}