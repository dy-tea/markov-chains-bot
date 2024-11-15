use poise::serenity_prelude as serenity;

use std::sync::{Arc, Mutex};

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

    // Load default model
    let default_model = postcard::from_bytes::<Model>(
        &std::fs::read(format!("{}/{}.model", MODEL_DIR, DEFAULT_MODEL_NAME))
            .expect("Failed to read model file"),
    )
    .expect("Failed to deserialize model");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), messages(), tokens(), dataset(), model(), query()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    params: Default::default(),
                    model: Arc::new(Mutex::new(default_model)),
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
