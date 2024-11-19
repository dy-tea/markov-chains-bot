use markov_chains::prelude::*;

use std::sync::Arc;
use tokio::sync::Mutex;

pub const MODEL_DIR: &str = "models";
pub const DEFAULT_MODEL_NAME: &str = "kleden4";
pub const MARKOV_CHAINS_VERSION: & str = "1.4.4";

pub struct Data {
    pub model: Arc<Mutex<Model>>,
    pub model_name: Arc<Mutex<String>>,
    pub params: Arc<Mutex<GenerationParams>>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
