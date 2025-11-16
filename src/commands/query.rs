use crate::{
    global::*,
    db::*
};

use markov_chains::prelude::*;
use unicode_width;

/// Send a query to the currently loaded model
#[poise::command(prefix_command, slash_command, broadcast_typing)]
pub async fn query(
    ctx: Context<'_>,
    #[description = "Starting query to run the current model off"] query: String,
    #[description = "Say as Cow"] cowsay: Option<bool>,
    #[description = "Wrap at column count"] wrap_at: Option<usize>,
) -> Result<(), Error> {
    let generated = ctx.say("Generating...").await?;

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
    
    // format the message as cowsay
    if let Some(cowsay) = cowsay {
        if cowsay {
            // Remove any code tags inside message
            let cleaned_message = message.replace("```", "");

            let cow = r#"     \   ^__^
      \  (oo)\_______
         (__)\       )\/\
             ||----w |
             ||     ||"#;

            // Helper function to calculate display width
            fn display_width(s: &str) -> usize {
                s.chars().map(|c| {
                    if c.len_utf8() > 1 {
                        unicode_width::UnicodeWidthChar::width(c).unwrap_or(1)
                    } else {
                        1
                    }
                }).sum()
            }
            
            // Wrap message at specified width or default to 40 characters
            let wrap_at = wrap_at.unwrap_or(40);
            let mut wrapped_message = String::new();
            let mut current_line = String::new();
            for word in cleaned_message.split_whitespace() {
                let word_width = display_width(word);
                let current_width = display_width(&current_line);
                if current_width + word_width + 1 > wrap_at {
                    if !wrapped_message.is_empty() {
                        wrapped_message.push('\n');
                    }
                    wrapped_message.push_str(&current_line);
                    current_line = word.to_string();
                } else {
                    if !current_line.is_empty() {
                        current_line.push(' ');
                    }
                    current_line.push_str(word);
                }
            }
            if !current_line.is_empty() {
                if !wrapped_message.is_empty() {
                    wrapped_message.push('\n');
                }
                wrapped_message.push_str(&current_line);
            }

            let max_line_length = wrapped_message.lines().map(|line| display_width(line)).max().unwrap_or(0);

            let mut bubble = String::new();
            if wrapped_message.lines().count() == 1 {
                // Single line message
                bubble.push_str(&format!(" {}\n", "-".repeat(max_line_length + 2)));
                let line = wrapped_message.lines().next().unwrap();
                let padding = max_line_length - display_width(line);
                bubble.push_str(&format!("| {}{} |\n", line, " ".repeat(padding)));
                bubble.push_str(&format!(" {}\n", "-".repeat(max_line_length + 2)));
            } else {
                // Multiple lines message
                bubble.push_str(&format!(" {}\n", "_".repeat(max_line_length + 2)));

                let line_count = wrapped_message.lines().count();
                for (i, line) in wrapped_message.lines().enumerate() {
                    let padding = max_line_length - display_width(line);
                    if i == 0 {
                        bubble.push_str(&format!("/ {}{} \\\n", line, " ".repeat(padding)));
                    } else if i == line_count - 1 {
                        bubble.push_str(&format!("\\ {}{} /\n", line, " ".repeat(padding)));
                    } else {
                        bubble.push_str(&format!("| {}{} |\n", line, " ".repeat(padding)));
                    }
                }

                bubble.push_str(&format!(" {}\n", "-".repeat(max_line_length + 2)));
            }
            bubble.push_str(cow);

            // Format as cowsay and enclose in markdown code tags
            let cowsay_message = format!("```\n{}\n```", bubble);

            generated.edit(ctx, poise::CreateReply{
                content: Some(cowsay_message),
                ..Default::default()
            }).await?;
            return Ok(());
        }
    }
    
    // Display the generated message
    generated.edit(ctx, poise::CreateReply {
        content: Some(message.clone()),
        ..Default::default()
    }).await?;

    Ok(())
}
