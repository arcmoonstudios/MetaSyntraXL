// src/controller.rs ~=#######D]====A===r===c====M===o===o===n====<Lord[CONTROLLER]Xyn>=====S===t===u====d===i===o===s====[R|$>

use crate::config::Config;
use crate::errors::MetaSyntraXLError;
use crate::transformer_rag::TransformerRAG;
use tch::{nn, Tensor};

pub struct Controller {
    transformer_rag: TransformerRAG,
}

impl Controller {
    pub fn new(vs: &nn::Path, config: &Config) -> Result<Self, MetaSyntraXLError> {
        let transformer_rag = TransformerRAG::new(vs, config)?;
        Ok(Self { transformer_rag })
    }

    pub async fn process(&self, input: &Tensor) -> Result<Tensor, MetaSyntraXLError> {
        self.transformer_rag.forward(input).await
    }
}