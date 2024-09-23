// src/cognitive_thought_entity.rs ~=#######D]====A===r===c====M===o===o===n====<Lord[COGNITIVE-THOUGHT-ENTITY]Xyn>=====S===t===u====d===i===o===s====[R|$>
use crate::ppo::PPO;
use crate::environment::Environment;
use std::sync::Arc;
use tch::{nn, Device, Tensor};
use tokio::sync::Mutex;

pub struct CognitiveThoughtEntity<'a> {
    pub state: Tensor,
    pub ppo: Arc<Mutex<PPO<'a>>>,
    pub genetic_code: Vec<f64>,
    pub fitness: f64,
}

impl<'a> CognitiveThoughtEntity<'a> {
    pub fn new(vs: &'a nn::VarStore, state_size: i64, action_size: i64) -> Self {
        Self {
            state: Tensor::zeros(&[state_size], (tch::Kind::Float, Device::Cpu)),
            ppo: Arc::new(Mutex::new(PPO::new(vs, state_size, action_size, 1e-3))),
            genetic_code: vec![0.0; 10],
            fitness: 0.0,
        }
    }

    pub async fn process(&mut self, input: &Tensor) -> Tensor {
        self.state = input.shallow_clone();

        let (action, log_prob) = {
            let ppo = self.ppo.lock().await;
            match ppo.act(&self.state) {
                Ok(result) => result,
                Err(_) => {
                    (Tensor::zeros(&[1], (tch::Kind::Float, Device::Cpu)), Tensor::zeros(&[1], (tch::Kind::Float, Device::Cpu)))
                }
            }
        };

        let mut env = Environment::new();

        let (next_state, reward, done) = env.step(&action);

        self.fitness += reward;

        if done {
            let returns = env.compute_returns();
            let state_value = {
                let ppo = self.ppo.lock().await;
                match ppo.evaluate(&self.state) {
                    Ok((_, value)) => value,
                    Err(_) => Tensor::zeros(&[1], (tch::Kind::Float, Device::Cpu)),
                }
            };
            let advantages = env.compute_advantages(&returns, &state_value);

            if let Err(e) = {
                let mut ppo = self.ppo.lock().await;
                ppo.learn(&self.state, &action, &log_prob, &returns, &advantages)
            } {
                eprintln!("Error during PPO update: {:?}", e);
            }
        }

        next_state
    }
}

impl<'a> Clone for CognitiveThoughtEntity<'a> {
    fn clone(&self) -> Self {
        CognitiveThoughtEntity {
            state: self.state.shallow_clone(),  // Use `shallow_clone()` for `Tensor`
            ppo: Arc::clone(&self.ppo),         // Clone the `Arc<Mutex<PPO>>`
            genetic_code: self.genetic_code.clone(),
            fitness: self.fitness,
        }
    }
}