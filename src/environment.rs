// src/environment.rs ~=#######D]====A===r===c====M===o===o===n====<Lord[ENVIRONMENT]Xyn>=====S===t===u====d===i===o===s====[R|$>
use tch::{Device, Kind, Tensor};

pub struct Environment {
    state: Tensor,
    done: bool,
    goal_state: Tensor,
    rewards: Vec<f64>,
    gamma: f64,
}

impl Environment {
    pub fn new() -> Self {
        let initial_state = Tensor::zeros(&[1], (Kind::Float, Device::Cpu));
        let goal_state = Tensor::from(10.0);
        Self {
            state: initial_state,
            done: false,
            goal_state,
            rewards: Vec::new(),
            gamma: 0.99, // Discount factor
        }
    }

    pub fn reset(&mut self) -> Tensor {
        self.state = Tensor::zeros(&[1], (Kind::Float, Device::Cpu));
        self.done = false;
        self.rewards.clear();
        self.state.shallow_clone()
    }

    pub fn step(&mut self, action: &Tensor) -> (Tensor, f64, bool) {
        let action_value = action.int64_value(&[]);

        self.state += Tensor::from(action_value as f32);

        let distance = (&self.goal_state - &self.state).abs().double_value(&[]);

        let reward = -distance;

        self.rewards.push(reward);

        if distance < 1e-3 {
            self.done = true;
        }

        (self.state.shallow_clone(), reward, self.done)
    }

    pub fn compute_returns(&self) -> Tensor {
        let mut returns = Vec::with_capacity(self.rewards.len());
        let mut g = 0.0;

        for &reward in self.rewards.iter().rev() {
            g = reward + self.gamma * g;
            returns.push(g);
        }

        returns.reverse();
        Tensor::of_slice(&returns).to_kind(Kind::Float)
    }

    pub fn compute_advantages(&self, returns: &Tensor, values: &Tensor) -> Tensor {
        returns - values
    }
}