use poise::serenity_prelude as serenity;

pub mod global;
pub use global::*;

pub mod messages;
pub mod tokens;
pub mod tokenized_messages;
pub mod ngram;
pub mod dataset;
pub mod model;

pub mod commands;
pub use commands::*;

pub mod prelude {
    pub use super::messages::Messages;

    pub use super::tokens::{
        Tokens,
        START_TOKEN,
        END_TOKEN
    };

    pub use super::tokenized_messages::TokenizedMessages;

    pub use super::ngram::{
        Ngram,
        Unigram,
        Bigram,
        Trigram
    };

    pub use super::dataset::Dataset;
    pub use super::model::params::GenerationParams;
    pub use super::model::transitions::Transitions;
    pub use super::model::generator::Generator;
    pub use super::model::model::Model;
}

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

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), messages(), tokens(), dataset(), model()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
