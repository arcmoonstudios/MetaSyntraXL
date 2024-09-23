// src/errors.rs ~=#######D]====A===r===c====M===o===o===n====<Lord[ERRORS]Xyn>=====S===t===u====d===i===o===s====[R|$>
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MetaSyntraXLError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Elasticsearch error: {0}")]
    ElasticsearchError(#[from] elasticsearch::Error),

    #[error("Tokio error: {0}")]
    TokioError(#[from] tokio::task::JoinError),

    #[error("Anyhow error: {0}")]
    AnyhowError(#[from] anyhow::Error),

    #[error("Transformer error: {0}")]
    TransformerError(String),

    #[error("Retrieval error: {0}")]
    RetrievalError(String),

    #[error("Thought Chain error: {0}")]
    ThoughtChainError(String),

    #[error("PPO error: {0}")]
    PPOError(String),

    #[error("Ensemble error: {0}")]
    EnsembleError(String),

    #[error("Evaluation error")]
    EvaluationError,

    #[error("Tch error: {0}")]
    TchError(String),
}