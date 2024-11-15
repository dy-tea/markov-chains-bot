use poise::serenity_prelude as serenity;

use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

use markov_chains::prelude::*;

pub mod global;
pub use global::*;

pub mod commands;
pub use commands::*;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

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
            commands: vec![age(), messages(), tokens(), dataset(), model(), params(), query()],
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
