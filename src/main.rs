// src/main.rs ~=#######D]====A===r===c====M===o===o===n====<Lord[MAIN]Xyn>=====S===t===u====d===i===o===s====[R|$>
use env_logger;
use log::{info, error};
use tch::{Tensor, nn};
use crate::{
    controller::Controller,
    errors::MetaSyntraXLError,
};

use crate::config::Config;

mod config;
mod transformer_rag;
mod transformer_model;
mod bayesian_network;
mod ppo;
mod cognitive_thought_entity;
mod thought_chain;
mod ensemble;
mod retrieval_system;
mod knowledge_graph;
mod environment;
mod tokenizer;
mod gradient_cache;
mod errors;
mod controller;

#[tokio::main]
async fn main() -> Result<(), MetaSyntraXLError> {
    env_logger::init();
    info!("Starting MetaSyntraXL...");

    let vs = nn::VarStore::new(tch::Device::Cpu);

    let config = Config::default();

    let controller = Controller::new(&vs.root(), &config)?;

    let input = Tensor::of_slice(&[1, 2, 3, 4])
        .to_kind(tch::Kind::Int64)
        .unsqueeze(0);

    match controller.process(&input).await {
        Ok(output) => info!("Processing successful: {:?}", output),
        Err(e) => error!("Processing failed: {:?}", e),
    }

    Ok(())
}