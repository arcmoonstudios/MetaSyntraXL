// src/config.rs ~=#######D]====A===r===c====M===o===o===n====<Lord[CONFIG]Xyn>=====S===t===u====d===i===o===s====[R|$>
#[derive(Debug, Clone)]
pub struct Config {
    pub vocab_size: i64,
    pub embed_dim: i64,
    pub num_heads: usize,
    pub hidden_dim: i64,
    pub num_layers: usize,
    pub max_len: usize,
    pub cache_capacity: usize,
    pub num_models: usize,
    pub input_size: i64,
    pub output_size: i64,
    pub dropout: f64,
    pub use_cuda: bool,
    pub elasticsearch: ElasticsearchConfig,
    pub prometheus: PrometheusConfig,
}

#[derive(Debug, Clone)]
pub struct ElasticsearchConfig {
    pub url: String,
    pub index: String,
}

#[derive(Debug, Clone)]
pub struct PrometheusConfig {
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
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
                index: "default_index".to_string(),
            },
            prometheus: PrometheusConfig {
                port: 9090,
            },
        }
    }
}