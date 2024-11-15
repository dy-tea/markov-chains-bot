use markov_chains::prelude::*;

use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

#[derive(Debug)]
pub enum GlobalData {
    Params(GenerationParams),
    Model(Model),
    ModelName(String),
}

pub const MODEL_DIR: &str = "models";
pub const DEFAULT_MODEL_NAME: &str = "kleden4";

pub struct Data {
    pub temp: Arc<Mutex<HashMap<&'static str, GlobalData>>>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
