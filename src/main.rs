use poise::serenity_prelude as serenity;

use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

use markov_chains::prelude::*;

pub mod global;
pub use global::*;

pub mod commands;
pub use commands::*;

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    // Create default temp data
    let mut temp = HashMap::new();

    // Load default model
    let default_model = postcard::from_bytes::<Model>(
        &std::fs::read(format!("{}/{}.model", MODEL_DIR, DEFAULT_MODEL_NAME))
            .expect("Failed to read model file"),
    )
    .expect("Failed to deserialize model");

    temp.insert("model", GlobalData::Model(default_model));
    temp.insert("params", GlobalData::Params(GenerationParams::default()));
    temp.insert("model_name", GlobalData::ModelName(DEFAULT_MODEL_NAME.to_string()));

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![/*messages(), tokens(), dataset(),*/ model(), params(), query()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    temp: Arc::new(Mutex::new(temp)),
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
