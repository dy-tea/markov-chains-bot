pub const DB_NAME: &str = "markov_chains.db";

pub const MODEL_DIR: &str = "models";

pub const DEFAULT_MODEL_NAME: &str = "kleden4";
pub const DEFAULT_MODEL_ID: u64 = 69; // FIXME

pub const MARKOV_CHAINS_VERSION: &str = "1.4.4";

pub struct Data {}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
