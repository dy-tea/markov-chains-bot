use crate::{
    global::*,
    db::*
};

use markov_chains::prelude::*;

/// Send a query to the currently loaded model
#[poise::command(prefix_command, slash_command, broadcast_typing)]
pub async fn query(
    ctx: Context<'_>,
    #[description = "Starting query to run the current model off"] query: String,
) -> Result<(), Error> {
    // Get the current user id
    let user_id = ctx.author().id.to_string();

    // Get loaded model id
    let loaded_model = user_get_loaded(user_id.parse().unwrap()).unwrap_or(DEFAULT_MODEL_ID.to_string());

    // Get the path to the model
    let model_path = format!("{}/{}", MODEL_DIR, loaded_model);

    // Load the model
    let model = match std::fs::read(model_path) {
        Ok(model) => {
            match postcard::from_bytes::<Model>(&model) {
                Ok(model) => model,
                Err(e) => return Err(format!("**ERROR:** `{}`", e).into())
            }
        }
        Err(e) => return Err(format!("**ERROR:** `{}`", e).into())
    };

    // Get user params
    let params = user_get_params(user_id.parse().unwrap()).unwrap();

    // Generate the current query
    let message_start = query.clone();
    let query = query.split_whitespace()
        .filter(|word| !word.is_empty())
        .map(|word| word.to_lowercase())
        .map(|word| {
            if word.starts_with("<:") && word.ends_with('>') {
                if let Some(end) = word.rfind(":") {
                    word[1..=end].to_string()
                } else {
                    word
                }
            } else {
                word
            }
        })
        .map(|word| model.tokens().find_token(&word))
        .collect::<Option<Vec<_>>>();

    let Some(query) = query else {
        return Err("**ERROR: Query not in dataset**".into());
    };

    if query.is_empty() {
        return Err("**ERROR: Query cannot be empty**".into());
    }

    let generated = ctx.say("Generating...").await?;
    let mut message = message_start;
    for token in model.generate(query, &params) {
        match token {
            Ok(token) => {
                let Some(word) = model.tokens().find_word(token) else {
                    return Err(format!("**ERROR: Failed to find word for token** `{}`", token).into());
                };

                message.push_str(" ");

                // Check if the word is an emoji
                if word.starts_with(':') && word.ends_with(':') {
                    // Try to find emoji in current server
                    match ctx.partial_guild().await {
                        Some(guild) => {
                            match guild.emojis(ctx.http()).await {
                                Ok(emojis) => {
                                    for emoji in emojis {
                                        if emoji.name == &word[1..word.len() - 1] {
                                            message.push_str(&format!("<:{}:{}>", emoji.name, emoji.id));
                                            break;
                                        }
                                    }
                                },
                                Err(e) =>
                                    return Err(format!("**ERROR:** `{}`", e).into()),
                            }
                        },
                        None => {
                            // Bot isn't being run from a server, return the emoji as is
                            message.push_str(word);
                        },
                    }
                } else {
                    message.push_str(word);
                }
            }
            Err(err) => {
                return Err(format!("**ERROR: Failed to generate** `{}`", err).into());
            }
        }
    }

    // Display the generated message
    generated.edit(ctx, poise::CreateReply {
        content: Some(message.clone()),
        ..Default::default()
    }).await?;

    Ok(())
}
