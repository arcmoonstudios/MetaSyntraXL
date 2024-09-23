// src/knowledge_graph.rs ~=#######D]====A===r===c====M===o===o===n====<Lord[KNOWLEDGE-GRAPH]Xyn>=====S===t===u====d===i===o===s====[R|$>

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct Entity {
    pub id: String,
    pub properties: HashMap<String, String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Relationship {
    pub from: String,
    pub to: String,
    pub type_: String,
}

pub struct KnowledgeGraph {
    entities: RwLock<HashMap<String, Entity>>,
    relationships: RwLock<Vec<Relationship>>,
}

impl KnowledgeGraph {
    /// Initializes a new, empty Knowledge Graph.
    pub fn new() -> Self {
        Self {
            entities: RwLock::new(HashMap::new()),
            relationships: RwLock::new(Vec::new()),
        }
    }

    /// Adds a new entity to the Knowledge Graph.
    pub async fn add_entity(&self, entity: Entity) {
        let mut entities = self.entities.write().await;
        entities.insert(entity.id.clone(), entity);
    }

    /// Adds a new relationship to the Knowledge Graph.
    pub async fn add_relationship(&self, relationship: Relationship) {
        let mut relationships = self.relationships.write().await;
        relationships.push(relationship);
    }

    /// Queries entities based on a property value.
    pub async fn query_entities_by_property(&self, key: &str, value: &str) -> Vec<Entity> {
        let entities = self.entities.read().await;
        entities
            .values()
            .filter(|e| e.properties.get(key).map_or(false, |v| v == value))
            .cloned()
            .collect()
    }

    /// Retrieves all relationships of a specific type.
    pub async fn get_relationships_by_type(&self, type_: &str) -> Vec<Relationship> {
        let relationships = self.relationships.read().await;
        relationships
            .iter()
            .filter(|r| r.type_ == type_)
            .cloned()
            .collect()
    }

    /// Retrieves all entities connected to a given entity via a specific relationship type.
    pub async fn get_connected_entities(&self, entity_id: &str, relationship_type: &str) -> Vec<Entity> {
        let relationships = self.relationships.read().await;
        let entities = self.entities.read().await;

        relationships
            .iter()
            .filter(|r| r.from == entity_id && r.type_ == relationship_type)
            .filter_map(|r| entities.get(&r.to).cloned())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_add_and_query_entity() {
        let kg = KnowledgeGraph::new();

        let mut properties = HashMap::new();
        properties.insert("type".to_string(), "person".to_string());
        properties.insert("name".to_string(), "Alice".to_string());

        let entity = Entity {
            id: "1".to_string(),
            properties,
        };

        kg.add_entity(entity.clone()).await;

        let results = kg.query_entities_by_property("name", "Alice").await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "1");
    }

    #[tokio::test]
    async fn test_add_and_query_relationship() {
        let kg = KnowledgeGraph::new();

        let entity1 = Entity {
            id: "1".to_string(),
            properties: HashMap::new(),
        };
        let entity2 = Entity {
            id: "2".to_string(),
            properties: HashMap::new(),
        };

        kg.add_entity(entity1.clone()).await;
        kg.add_entity(entity2.clone()).await;

        let relationship = Relationship {
            from: "1".to_string(),
            to: "2".to_string(),
            type_: "friend".to_string(),
        };

        kg.add_relationship(relationship.clone()).await;

        let relationships = kg.get_relationships_by_type("friend").await;
        assert_eq!(relationships.len(), 1);
        assert_eq!(relationships[0].from, "1");
        assert_eq!(relationships[0].to, "2");
    }

    #[tokio::test]
    async fn test_get_connected_entities() {
        let kg = KnowledgeGraph::new();

        let entity1 = Entity {
            id: "1".to_string(),
            properties: HashMap::new(),
        };
        let entity2 = Entity {
            id: "2".to_string(),
            properties: HashMap::new(),
        };
        let entity3 = Entity {
            id: "3".to_string(),
            properties: HashMap::new(),
        };

        kg.add_entity(entity1.clone()).await;
        kg.add_entity(entity2.clone()).await;
        kg.add_entity(entity3.clone()).await;

        let relationship1 = Relationship {
            from: "1".to_string(),
            to: "2".to_string(),
            type_: "colleague".to_string(),
        };
        let relationship2 = Relationship {
            from: "1".to_string(),
            to: "3".to_string(),
            type_: "friend".to_string(),
        };

        kg.add_relationship(relationship1.clone()).await;
        kg.add_relationship(relationship2.clone()).await;

        let connected = kg.get_connected_entities("1", "colleague").await;
        assert_eq!(connected.len(), 1);
        assert_eq!(connected[0].id, "2");

        let connected = kg.get_connected_entities("1", "friend").await;
        assert_eq!(connected.len(), 1);
        assert_eq!(connected[0].id, "3");
    }
}