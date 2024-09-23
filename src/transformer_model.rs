// src/transformer_model.rs ~=#######D]====A===r===c====M===o===o===n====<Lord[TRANSFORMER-MODEL]Xyn>=====S===t===u====d===i===o===s====[R|$>
use crate::config::Config;
use tch::nn::{self, Module, Path};
use tch::{Kind, Tensor};

pub struct TransformerModel {
    embedding: nn::Embedding,
    positional_embedding: nn::Embedding,
    encoder_layers: Vec<EncoderLayer>,
    layer_norm: nn::LayerNorm,
    output_layer: nn::Linear,
}

impl TransformerModel {
    pub fn new(vs: &Path, config: &Config) -> Self {
        let embedding = nn::embedding(
            vs / "embedding",
            config.vocab_size,
            config.embed_dim,
            Default::default(),
        );

        let positional_embedding = nn::embedding(
            vs / "positional_embedding",
            config.max_len as i64,
            config.embed_dim,
            Default::default(),
        );

        let mut encoder_layers = Vec::new();
        for i in 0..config.num_layers {
            let layer_vs = &(vs / format!("encoder_layer_{}", i));
            let layer = EncoderLayer::new(layer_vs, config);
            encoder_layers.push(layer);
        }

        let layer_norm = nn::layer_norm(
            vs / "layer_norm",
            vec![config.embed_dim],
            Default::default(),
        );

        let output_layer = nn::linear(
            vs / "output_layer",
            config.embed_dim,
            config.vocab_size,
            Default::default(),
        );

        Self {
            embedding,
            positional_embedding,
            encoder_layers,
            layer_norm,
            output_layer,
        }
    }

    pub fn forward(&self, input: &Tensor) -> Tensor {
        let batch_size = input.size()[0];
        let seq_length = input.size()[1];

        let position_ids = Tensor::arange(seq_length, (Kind::Int64, input.device()))
            .unsqueeze(0)
            .expand(&[batch_size, seq_length], false);

        let token_embeddings = self.embedding.forward(input);
        let position_embeddings = self.positional_embedding.forward(&position_ids);

        let mut embeddings = token_embeddings + position_embeddings;

        for layer in &self.encoder_layers {
            embeddings = layer.forward(&embeddings);
        }

        let normalized_output = self.layer_norm.forward(&embeddings);

        let logits = self.output_layer.forward(&normalized_output);

        logits
    }
}

pub struct EncoderLayer {
    linear1: nn::Linear,
    linear2: nn::Linear,
    norm1: nn::LayerNorm,
    dropout: f64,
}

impl EncoderLayer {
    pub fn new(vs: &Path, config: &Config) -> Self {
        let linear1 = nn::linear(
            vs / "linear1",
            config.embed_dim,
            config.hidden_dim,
            Default::default(),
        );
        let linear2 = nn::linear(
            vs / "linear2",
            config.hidden_dim,
            config.embed_dim,
            Default::default(),
        );

        let norm1 = nn::layer_norm(vs / "norm1", vec![config.embed_dim], Default::default());

        Self {
            linear1,
            linear2,
            norm1,
            dropout: config.dropout,
        }
    }

    pub fn forward(&self, x: &Tensor) -> Tensor {
        let ff_output = self
            .linear2
            .forward(&self.linear1.forward(&x).relu())
            .dropout(self.dropout, /*train=*/true);

        let x = x + ff_output;
        let x = self.norm1.forward(&x);

        x
    }
}