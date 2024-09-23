// src/ppo.rs ~=#######D]====A===r===c====M===o===o===n====<Lord[PPO]Xyn>=====S===t===u====d===i===o===s====[R|$>
use crate::errors::MetaSyntraXLError;
use tch::{nn, nn::Module, nn::OptimizerConfig, Kind, Tensor};

pub struct PPO<'a> {
    vs: &'a nn::VarStore,
    policy: nn::Sequential,
    value: nn::Sequential,
    optimizer: nn::Optimizer,
    clip_param: f64,
    max_grad_norm: f64,
}

impl<'a> PPO<'a> {
    pub fn new(vs: &'a nn::VarStore, state_dim: i64, action_dim: i64, lr: f64) -> Self {
        let policy = nn::seq()
            .add(nn::linear(
                vs.root() / "p1",
                state_dim,
                64,
                Default::default(),
            ))
            .add_fn(|x| x.relu())
            .add(nn::linear(
                vs.root() / "p2",
                64,
                action_dim,
                Default::default(),
            ))
            .add_fn(|x| x.softmax(-1, Kind::Float));

        let value = nn::seq()
            .add(nn::linear(
                vs.root() / "v1",
                state_dim,
                64,
                Default::default(),
            ))
            .add_fn(|x| x.relu())
            .add(nn::linear(
                vs.root() / "v2",
                64,
                1,
                Default::default(),
            ));

        let optimizer = nn::Adam::default().build(vs, lr).unwrap();

        Self {
            vs,
            policy,
            value,
            optimizer,
            clip_param: 0.2,
            max_grad_norm: 0.5,
        }
    }

    pub fn act(&self, state: &Tensor) -> Result<(Tensor, Tensor), MetaSyntraXLError> {
        let action_probs = self.policy.forward(state);
        let action = action_probs.multinomial(1, true);
        let log_prob = action_probs
            .log_softmax(-1, Kind::Float)
            .gather(-1, &action, false)
            .squeeze();
        Ok((action, log_prob))
    }

    pub fn evaluate(&self, states: &Tensor) -> Result<(Tensor, Tensor), MetaSyntraXLError> {
        let action_probs = self.policy.forward(states); 
        let state_value = self.value.forward(states); 
        Ok((action_probs, state_value))
    }

    pub fn learn(
        &mut self, 
        states: &Tensor, 
        actions: &Tensor, 
        log_probs: &Tensor, 
        returns: &Tensor, 
        advantages: &Tensor
    ) -> Result<(), MetaSyntraXLError> {
        let (_action_probs, _state_values) = self.evaluate(states).map_err(|e| {
            eprintln!("Error during evaluate: {:?}", e);
            MetaSyntraXLError::EvaluationError // Corrected error variant
        })?;

        self.update(states, actions, log_probs, returns, advantages)?;

        Ok(())
    }

    pub fn update(
        &mut self,
        states: &Tensor,
        actions: &Tensor,
        old_log_probs: &Tensor,
        returns: &Tensor,
        advantages: &Tensor,
    ) -> Result<(), MetaSyntraXLError> {
        let (action_probs, state_values) = self.evaluate(states)?;

        let new_log_probs = action_probs
            .log_softmax(-1, Kind::Float)
            .gather(-1, actions, false)
            .squeeze();

        let ratio = (new_log_probs - old_log_probs).exp();

        let surr1 = &ratio * advantages;
        let surr2 = ratio
            .clamp(1.0 - self.clip_param, 1.0 + self.clip_param)
            * advantages;
        let actor_loss = -surr1.minimum(&surr2).mean(Kind::Float);

        let critic_loss = (returns - state_values)
            .pow(&Tensor::from(2.0))
            .mean(Kind::Float);

        let loss: Tensor = actor_loss + 0.5 * critic_loss;

        self.optimizer.zero_grad();
        loss.backward();
        self.optimizer.step();

        Ok(())
    }
}