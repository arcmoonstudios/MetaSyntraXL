// src/ensemble.rs ~=#######D]====A===r===c====M===o===o===n====<Lord[ENSEMBLE]Xyn>=====S===t===u====d===i===o===s====[R|$>
use crate::config::Config;
use crate::errors::MetaSyntraXLError;
use crate::transformer_rag::TransformerRAG;
use futures::future::join_all;
use tch::{nn, Kind, Tensor};

pub struct Ensemble {
    models: Vec<TransformerRAG>,
    #[allow(dead_code)]
    meta_model: nn::Sequential,
}

impl Ensemble {
    pub fn new(
        vs: &nn::Path,
        num_models: usize,
        _input_size: i64,
        output_size: i64,
        config: &Config,
    ) -> Result<Self, MetaSyntraXLError> {
        let mut models = Vec::new();
        for i in 0..num_models {
            let model_vs = vs.sub(&format!("ensemble_model{}", i));
            let model = TransformerRAG::new(&model_vs, config)?;
            models.push(model);
        }

        let meta_model = nn::seq()
            .add(nn::linear(
                vs / "meta1",
                num_models as i64 * output_size,
                64,
                Default::default(),
            ))
            .add_fn(|x| x.relu())
            .add(nn::linear(vs / "meta2", 64, output_size, Default::default()));

        Ok(Self { models, meta_model })
    }

    pub async fn bagging_predict(&self, input: &Tensor) -> Result<Tensor, MetaSyntraXLError> {
        let predictions = join_all(self.models.iter().map(|model| model.forward(input))).await;
        let valid_predictions: Result<Vec<Tensor>, MetaSyntraXLError> =
            predictions.into_iter().collect();
        let stacked_predictions = Tensor::stack(&valid_predictions?, 0);
        Ok(stacked_predictions.mean_dim(
            Some(&[0i64][..]),
            false,
            Kind::Float,
        ))
    }
}