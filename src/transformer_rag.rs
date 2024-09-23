// src/transformer_rag.rs ~=#######D]====A===r===c====M===o===o===n====<Lord[TRANSFORMER-RAG]Xyn>=====S===t===u====d===i===o===s====[R|$>
use crate::config::Config;
use crate::errors::MetaSyntraXLError;
use crate::retrieval_system::RetrievalSystem;
use crate::tokenizer::Tokenizer;
use crate::transformer_model::TransformerModel;
use tch::nn::Path;
use tch::{Device, Kind, Tensor};

pub struct TransformerRAG {
    transformer: TransformerModel,
    retrieval_system: RetrievalSystem,
    tokenizer: Tokenizer,
    device: Device,
}

impl TransformerRAG {
    pub fn new(vs: &Path, config: &Config) -> Result<Self, MetaSyntraXLError> {
        let tokenizer = Tokenizer::new();
        let retrieval_system = RetrievalSystem::new(config)?;
        let device = if config.use_cuda && Device::cuda_if_available().is_cuda() {
            Device::Cuda(0)
        } else {
            Device::Cpu
        };

        let transformer = TransformerModel::new(vs, config);

        Ok(Self {
            transformer,
            retrieval_system,
            tokenizer,
            device,
        })
    }

    pub async fn forward(&self, input: &Tensor) -> Result<Tensor, MetaSyntraXLError> {
        let input = input.to_device(self.device);

        let input_tokens: Vec<i64> = input
            .to_kind(Kind::Int64)
            .flatten(0, -1)
            .iter::<i64>()
            .map_err(|e| MetaSyntraXLError::TchError(e.to_string()))?
            .collect();

        let input_text = self.tokenizer.decode(&input_tokens);

        let retrieved_docs = self.retrieval_system.retrieve(&input_text).await?;

        let augmented_input = format!(
            "{} {}",
            input_text,
            retrieved_docs.iter().map(|doc| doc.clone()).collect::<Vec<String>>().join(" ")
        );

        let augmented_tokens = self.tokenizer.encode(&augmented_input);

        let augmented_tensor = Tensor::of_slice(&augmented_tokens)
            .to_kind(Kind::Int64)
            .unsqueeze(0)
            .to_device(self.device);

        let output = self.transformer.forward(&augmented_tensor);

        Ok(output)
    }
}