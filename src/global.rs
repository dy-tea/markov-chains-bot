use markov_chains::prelude::*;

use std::sync::{Arc, Mutex};

pub const MODEL_DIR: &str = "models";
pub const DEFAULT_MODEL_NAME: &str = "kleden4";

pub struct Data {
    pub params: Arc<Mutex<GenerationParams>>,
    pub model: Arc<Mutex<Model>>,
    pub model_name: Arc<Mutex<String>>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
