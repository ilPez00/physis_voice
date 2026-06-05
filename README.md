# Physis Voice 🎙️🧠

> **⚠️ Folded into `physis_pro`.** This standalone project is superseded by the
> `physis-pro-voice` binary (`physis_pro/src/bin/voice.rs`), which runs the same
> pipeline on the shared engine (provider cascade, ontology classifier, Dream
> Function, multimodal sensory bus). Use that for new work; see
> `physis_pro/README.md` → "Multimodal Sensory". This repo is kept for reference.

Physis Voice is a real-time ontological mapping engine designed for live conferences and lectures. It captures audio, transcribes it, and uses AI to map concepts, entities, and relationships into an interactive graph.

## Features

- **Live Transcription**: Captures audio in 15s chunks using `arecord`.
- **Ontological Extraction**: Automatically identifies entities (people, topics, concepts) and their connections.
- **Interactive Visualization**: Real-time graph UI powered by Cytoscape.js.
- **Offline Intelligence**: Uses local **Ollama** (`llama3.2`) for concept mapping to ensure privacy and offline capability.
- **Hacker-style UI**: Green-on-black monospace dashboard at `http://localhost:3000`.

## Prerequisites

- **Linux** (uses `arecord` from `alsa-utils`).
- **Rust** (Cargo).
- **Ollama** (for local AI mapping).
  - Run `ollama pull llama3.2`
- **Groq API Key** (for fast transcription).

## Installation & Setup

1. **Clone and Setup Environment**:
   ```bash
   git clone <repo-url>
   cd physis_voice
   ```
2. **Configure Keys**:
   Create a `.env` file or export your keys:
   ```bash
   AI_GROQ_KEY=your_key_here
   ```

## Usage

1. **Start the Engine**:
   ```bash
   cargo run --release
   ```
2. **View the Map**:
   Open [http://localhost:3000](http://localhost:3000) in your browser.

## Architecture

- **`src/main.rs`**: Core loop and Axum web server.
- **`src/physis.rs`**: Ontological data structures and merging logic.
- **`src/ai/`**: Provider cascade for routing tasks between local and cloud AI.
- **`src/index.html`**: Cytoscape.js frontend.

## License

MIT
