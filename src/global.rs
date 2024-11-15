use markov_chains::prelude::*;

use std::sync::{Arc, Mutex};

pub const MODEL_DIR: &str = "models";

pub struct Data {
    pub params: Arc<Mutex<GenerationParams>>,
    pub model: Arc<Mutex<Model>>
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
