// src/thought_chain.rs ~=#######D]====A===r===c====M===o===o===n====<Lord[THOUGHT-CHAIN]Xyn>=====S===t===u====d===i===o===s====[R|$>
use crate::cognitive_thought_entity::CognitiveThoughtEntity;
use crate::errors::MetaSyntraXLError;
use std::sync::Arc;
use tch::nn;
use tokio::sync::Mutex;
use rand::seq::SliceRandom;

pub struct ThoughtChain<'a> {
    entities: Vec<Arc<Mutex<CognitiveThoughtEntity<'a>>>>,
    elite_fraction: f64,
}

impl<'a> ThoughtChain<'a> {
    pub fn new(
        vs: &'a nn::VarStore,
        num_entities: usize,
        state_size: i64,
        action_size: i64,
        elite_fraction: f64,
    ) -> Self {
        let mut entities = Vec::new();
        for _ in 0..num_entities {
            let cte = CognitiveThoughtEntity::new(vs, state_size, action_size); 
            entities.push(Arc::new(Mutex::new(cte)));
        }

        Self {
            entities,
            elite_fraction,
        }
    }

    pub async fn process(&self, input: &tch::Tensor) -> Result<tch::Tensor, MetaSyntraXLError> {
        let mut current_input = input.shallow_clone();
        for entity in &self.entities {
            let mut cte = entity.lock().await;
            current_input = cte.process(&current_input).await;
        }
        Ok(current_input)
    }

    pub async fn evolve(&mut self, _vs: &'a nn::VarStore) -> Result<(), MetaSyntraXLError> {
        self.entities.sort_by(|a, b| {
            let a_fitness = futures::executor::block_on(a.lock()).fitness;
            let b_fitness = futures::executor::block_on(b.lock()).fitness;
            b_fitness
                .partial_cmp(&a_fitness)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let elite_count = (self.entities.len() as f64 * self.elite_fraction).ceil() as usize;
        let elites = self.entities[..elite_count].to_vec();

        let mut new_entities = elites.clone();

        while new_entities.len() < self.entities.len() {
            let parent = elites
                .choose(&mut rand::thread_rng())
                .ok_or(MetaSyntraXLError::ThoughtChainError(
                    "No elite entities available for reproduction".to_string(),
                ))?;
            let parent_clone = {
                let parent_guard = parent.lock().await;
                Arc::new(Mutex::new((*parent_guard).clone()))  // Using `clone()` after `Clone` is implemented
            };
            new_entities.push(parent_clone);
        }

        self.entities = new_entities;
        Ok(())
    }

    pub fn add_entity(&mut self, entity: Arc<Mutex<CognitiveThoughtEntity<'a>>>) {
        self.entities.push(entity);
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}