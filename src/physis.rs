use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct OntologicalMap {
    pub entities: HashMap<String, Entity>,
    pub relationships: Vec<Relationship>,
}

impl OntologicalMap {
    pub fn merge(&mut self, other: OntologicalMap) {
        self.entities.extend(other.entities);
        self.relationships.extend(other.relationships);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub description: String,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Relationship {
    pub source: String,
    pub target: String,
    pub predicate: String,
    pub weight: f64,
}
