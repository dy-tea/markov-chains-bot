use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;

use poise::serenity_prelude as serenity;

pub mod global;
use global::*;

pub mod db;
use db::create_db;

pub mod commands;
pub use commands::*;

pub mod utils;

#[tokio::main]
async fn main() {
    // Get discord token
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");

    // Create db if not exists
    create_db().expect("ERROR: Failed to create db");

    // Create serenity framework
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![/*messages(), tokens(), dataset(),*/ model(), params(), query(), sysinfo()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    queue: Arc::new(Mutex::new(VecDeque::new()))
                })
            })
        })
        .build();

    // Load serenity client
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT | serenity::GatewayIntents::GUILD_EMOJIS_AND_STICKERS;
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
