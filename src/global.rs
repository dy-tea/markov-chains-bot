use markov_chains::prelude::Model;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

pub const DB_NAME: &str = "markov_chains.db";

pub const MODEL_DIR: &str = "models";

pub const DEFAULT_MODEL_NAME: &str = "kleden4";
pub const DEFAULT_MODEL_ID: &str = "16291105416022699669";

pub const MARKOV_CHAINS_VERSION: &str = "1.4.4";

pub struct Data {
    pub queue: Arc<Mutex<VecDeque<(u64, u64)>>>,
    pub models: Arc<RwLock<HashMap<String, Arc<Model>>>>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
