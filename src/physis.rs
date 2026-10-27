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
        for rel in other.relationships {
            let exists = self.relationships.iter().any(|r| {
                r.source == rel.source && r.target == rel.target && r.predicate == rel.predicate
            });
            if !exists {
                self.relationships.push(rel);
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn rel(source: &str, target: &str, predicate: &str) -> Relationship {
        Relationship {
            source: source.into(),
            target: target.into(),
            predicate: predicate.into(),
            weight: 1.0,
        }
    }

    #[test]
    fn merge_dedupes_identical_relationships() {
        let mut map = OntologicalMap::default();
        let mut chunk = OntologicalMap::default();
        chunk
            .relationships
            .push(rel("photosynthesis", "chlorophyll", "requires"));
        chunk
            .relationships
            .push(rel("photosynthesis", "chlorophyll", "requires"));

        map.merge(chunk.clone());
        map.merge(chunk);

        assert_eq!(map.relationships.len(), 1);
    }

    #[test]
    fn merge_keeps_distinct_relationships() {
        let mut map = OntologicalMap::default();
        let mut chunk = OntologicalMap::default();
        chunk.relationships.push(rel("a", "b", "requires"));
        chunk.relationships.push(rel("a", "b", "produces"));
        chunk.relationships.push(rel("a", "c", "requires"));

        map.merge(chunk);

        assert_eq!(map.relationships.len(), 3);
    }
}
