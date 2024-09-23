// src/tests.rs ~=#######D]====A===r===c====M===o===o===n====<Lord[TESTS]Xyn>=====S===t===u====d===i===o===s====[R|$>
use super::*;
use tokio::runtime::Runtime;
use tch::Tensor;
use crate::controller::Controller;
use crate::errors::MetaSyntraXLError;
use crate::transformer_rag::TransformerRAG;
use crate::ensemble::Ensemble;
use crate::gradient_cache::GradientCache;
use crate::config::{Config, ElasticsearchConfig, PrometheusConfig};
use std::collections::HashMap;  

#[tokio::test]
async fn test_transformer_rag_forward() -> Result<(), MetaSyntraXLError> {
    let vs = tch::nn::VarStore::new(tch::Device::Cpu);
    let config = Config {
        vocab_size: 10000,
        embed_dim: 512,
        num_heads: 8,
        hidden_dim: 2048,
        num_layers: 6,
        max_len: 512,
        cache_capacity: 1000,
        num_models: 3,
        input_size: 512,
        output_size: 10,
        dropout: 0.1,
        use_cuda: false,
        elasticsearch: ElasticsearchConfig {
            url: "http://localhost:9200".to_string(),
            index: "test_index".to_string(),
        },
        prometheus: PrometheusConfig {
            port: 9090,
        },
    };
    let transformer_rag = TransformerRAG::new(&vs.root(), &config)?;
    let input = Tensor::of_slice(&[1, 2, 3, 4]).unsqueeze(0);
    let output = transformer_rag.forward(&input).await?;
    assert_eq!(output.size(), &[1, config.max_len as i64, config.vocab_size]);
    Ok(())
}

#[tokio::test]
async fn test_ensemble_bagging() -> Result<(), MetaSyntraXLError> {
    let vs = tch::nn::VarStore::new(tch::Device::Cpu);
    let config = Config {
        vocab_size: 10000,
        embed_dim: 512,
        num_heads: 8,
        hidden_dim: 2048,
        num_layers: 6,
        max_len: 512,
        cache_capacity: 1000,
        num_models: 5,
        input_size: 512,
        output_size: 10,
        dropout: 0.1,
        use_cuda: false,
        elasticsearch: ElasticsearchConfig {
            url: "http://localhost:9200".to_string(),
            index: "test_index".to_string(),
        },
        prometheus: PrometheusConfig {
            port: 9090,
        },
    };
    let ensemble = Ensemble::new(
        &vs.root(),
        config.num_models,
        config.input_size,
        config.output_size,
        &config,
    )?;
    let input = Tensor::of_slice(&[1.0; 512]).unsqueeze(0);
    let prediction = ensemble.bagging_predict(&input).await?;
    assert_eq!(prediction.size(), &[512]);
    Ok(())
}