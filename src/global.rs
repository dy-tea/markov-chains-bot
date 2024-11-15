use markov_chains::prelude::GenerationParams;

use std::sync::{Arc, Mutex};

pub struct Data {
    pub params: Arc<Mutex<GenerationParams>>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
