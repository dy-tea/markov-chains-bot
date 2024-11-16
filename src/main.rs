use poise::serenity_prelude as serenity;

use std::sync::Arc;
use tokio::sync::Mutex;

use markov_chains::prelude::*;

pub mod global;
pub use global::*;

pub mod commands;
pub use commands::*;

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    // Load default model
    let default_model = postcard::from_bytes::<Model>(
        &std::fs::read(format!("{}/{}.model", MODEL_DIR, DEFAULT_MODEL_NAME))
            .expect("Failed to read model file"),
    )
    .expect("Failed to deserialize model");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![/*messages(), tokens(), dataset(),*/ model(), params(), query()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    model: Arc::new(Mutex::new(default_model)),
                    params: Arc::new(Mutex::new(GenerationParams::default())),
                    model_name: Arc::new(Mutex::new(DEFAULT_MODEL_NAME.to_string())),
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
