use crate::{
    global::*,
    db::*,
    utils::pretty_bytes
};

use poise::serenity_prelude as serenity;
use reqwest::header::CONTENT_DISPOSITION;
use chrono::Local;
use bytes::Bytes;
use std::fs::read_dir;
use xxh3::hash64_with_seed;

use markov_chains::{
    prelude::*,
    dataset::Dataset,
};

// download file at the specfied url, return the name and content
pub async fn download(url: String) -> Result<(String, Bytes), Error> {
    let response = reqwest::get(&url).await;

    match response {
        Ok(response) => {
            // Prase name from headers
            let name = response
                .headers()
                .get(CONTENT_DISPOSITION)
                .and_then(|header| header.to_str().ok())
                .and_then(|header_str| {
                    header_str
                        .split(';')
                        .find_map(|part| {
                            if part.trim().starts_with("filename=") {
                                part.trim().strip_prefix("filename=").map(|s| s.trim_matches('"').to_string())
                            } else {
                                None
                            }
                        })
                })
                .unwrap_or_else(|| {
                    let now = Local::now();
                    format!("{}", now.format("%Y-%m-%d-%H-%M-%S"))
                });

            // Return the content
            let content = response.bytes().await.unwrap();
            Ok((name, content))
        },
        Err(e) => Err(e.into()),
    }
}

#[poise::command(prefix_command, slash_command, subcommand_required, subcommands(/*"build", */"fromscratch", "load", "list", "info"))]
pub async fn model(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/*
/// Build language model
#[poise::command(prefix_command, slash_command)]
pub async fn build(ctx: Context<'_>,
    #[description = "Dataset bundle file"] file: Option<serenity::Attachment>,
    #[description = "Link to dataset bundle"] url: Option<String>,
    #[description = "Build bigrams transitions table"] bigrams: bool,
    #[description = "Build trigrams transitions table"] trigrams: bool,
    #[description = "Model name (overwrites file name if provided)"] model_name: Option<String>,
    #[description = "Model description"] description: Option<String>,
) -> Result<(), Error> {
    let status = ctx.say("Finding model type...").await?;

    // Get name and content of dataset
    let (name, content) = {
        if let Some(url) = url {
            // Download dataset from url
            status.edit(ctx, poise::CreateReply {
                content: Some(format!("Downloading model from url {}", url)),
                ..Default::default()
            }).await?;

            let content = download(url).await.unwrap();

            (content.0, content.1.to_vec())
        } else if let Some(file) = file {
            // Download dataset from attachment
            status.edit(ctx, poise::CreateReply {
                content: Some(format!("Downloading model from attachment {}", file.filename)),
                ..Default::default()
            }).await?;

            let name = file.filename.clone();
            let content = file.download().await?;

            (name, content)
        }
        else {
            return Err("**ERROR: No attachment or url provided**".into())
        }
    };

    // Load dataset from content
    let Ok(dataset) = postcard::from_bytes::<Dataset>(&content) else {
        return Err("**ERROR: Invalid dataset".into());
    };

    // Get optional model name
    let model_name = if let Some(mn) = model_name {
        mn.split('.').next().unwrap_or(&mn).to_owned()
    } else {
        name.split('.').next().unwrap_or(&name).to_owned()
    };

    // Create model from dataset
    let mut model = Model::build(dataset, bigrams, trigrams)
        .with_header("name", model_name.clone())
        .with_header("version", MARKOV_CHAINS_VERSION);

    // Add optional description
    if let Some(description) = description {
        model = model.with_header("description", description);
    }

    // Write model to file
    std::fs::write(format!("{}/{}.model", MODEL_DIR, model_name), postcard::to_allocvec(&model)?)?;

    // Update user
    status.edit(ctx, poise::CreateReply {
        content: Some(format!("Model `{}` built successfully", model_name)),
        ..Default::default()
    }).await?;

    Ok(())
}*/

/// Build language model from plain messages files
#[poise::command(prefix_command, slash_command)]
pub async fn fromscratch(
    ctx: Context<'_>,
    #[description = "Plain messages file"] file: Option<serenity::Attachment>,
    #[description = "Link to the plain messages file"] url: Option<String>,
    #[description = "Build bigrams transitions table"] bigrams: bool,
    #[description = "Build trigrams transitions table"] trigrams: bool,
    #[description = "Model name (overwrites file name if provided)"] model_name: Option<String>,
    #[description = "Model description"] description: Option<String>,
) -> Result<(), Error> {
    let status = ctx.say("Finding model type...").await?;

    // Get name and content of plain messages file
    let (name, content) = {
        if let Some(url) = url {
            // Download messages from url
            status.edit(ctx, poise::CreateReply {
                content: Some(format!("Attempting to download file from url {}", url)),
                ..Default::default()
            }).await?;

            let Ok((name, content)) = download(url.clone()).await else {
                return Err("**ERROR: Failed to download url**".into());
            };

            status.edit(ctx, poise::CreateReply {
                content: Some(format!("Successfully downloaded file {}", url)),
                ..Default::default()
            }).await?;

            (name, content.to_vec())
        } else if let Some(file) = file {
            // Download dataset from attachment
            status.edit(ctx, poise::CreateReply {
                content: Some(format!("Downloading model from attachment {}", file.url)),
                ..Default::default()
            }).await?;

            let name = file.filename.clone();
            let Ok(content) = file.download().await else {
                return Err("**ERROR: Failed to download attachment**".into());
            };

            status.edit(ctx, poise::CreateReply {
                content: Some(format!("Successfully downloaded attachment {}", file.url)),
                ..Default::default()
            }).await?;

            (name, content)
        } else {
            return Err("**ERROR: No attachment or url provided**".into())
        }
    };

    status.edit(ctx, poise::CreateReply {
        content: Some(format!("Attempting to build model from file {}", name)),
        ..Default::default()
    }).await?;

    // Convert content to string array
    let content = String::from_utf8(content.to_vec())?
        .split('\n').map(|s| s.to_string())
        .collect::<Vec<String>>();

    // Parse messages from input file
    let messages = Messages::parse_from_lines(&content);

    // Parse tokens from messages
    let tokens = Tokens::parse_from_messages(&messages);

    // Tokenize messages
    let tokenized_messages = TokenizedMessages::tokenize_message(&messages, &tokens)?;

    // Create the dataset
    let dataset = Dataset::default()
        .with_messages(tokenized_messages, 1)
        .with_tokens(tokens);

    // Get the name
    let name: String = {
        let a = model_name.unwrap_or(name);
        a.split('.').next().unwrap_or(&a).to_string()
    };

    // Get the current time
    let now = Local::now();

    // Get the model id
    let id = hash64_with_seed(name.as_bytes(), now.timestamp() as u64);

    // Build the model
    let mut model = Model::build(dataset, bigrams, trigrams)
        .with_header("name", name.clone())
        .with_header("model_id", id)
        .with_header("created_at", now)
        .with_header("version", MARKOV_CHAINS_VERSION);

    // Add optional description
    if let Some(description) = description {
        model = model.with_header("description", description);
    }

    // Store the model
    std::fs::write(format!("{}/{}", MODEL_DIR, id), postcard::to_allocvec(&model)?)?;

    // Add to database
    add_model(id.to_string(), name.clone()).unwrap();

    // Set the model as completed
    status.edit(ctx, poise::CreateReply {
        content: Some(format!("Successfully build model `{}`", name)),
        ..Default::default()
    }).await?;

    Ok(())
}

/// Load language model
#[poise::command(prefix_command, slash_command)]
pub async fn load(
    ctx: Context<'_>,
    #[description = "Name of model to load (e.g. kleden4)"] name: Option<String>,
    #[description = "Model file"] file: Option<serenity::Attachment>,
    #[description = "Link to the model"] url: Option<String>,
) -> Result<(), Error> {
    let status = ctx.say("Attempting to load model").await?;

    let (id, name) = {
        let now = Local::now().timestamp();

        if let Some(name) = name {
            (model_get_id(name.clone())?, name)
        } else if let Some(file) = file {
            let Ok(content) = file.download().await else {
                return Err("**ERROR: Failed to download attachment**".into());
            };

            let Ok(model) = postcard::from_bytes::<Model>(&content) else {
                return Err("**ERROR: Invalid model file** ".into());
            };
            let name = model.headers().get("name").unwrap_or(&file.filename);

            let id = hash64_with_seed(name.as_bytes(), now as u64);
            std::fs::write(format!("{}/{}", MODEL_DIR, id), content)?;

            add_model(id.to_string(), name.clone()).unwrap();

            (id, name.to_string())
        } else if let Some(url) = url {
            let Ok((name, content)) = download(url.clone()).await else {
                return Err("**ERROR: Failed to download url**".into());
            };

            let Ok(model) = postcard::from_bytes::<Model>(&content) else {
                return Err("**ERROR: Invalid model file**".into());
            };
            let name = model.headers().get("name").unwrap_or(&name);

            let id = hash64_with_seed(name.as_bytes(), now as u64);
            std::fs::write(format!("{}/{}", MODEL_DIR, id), content)?;

            add_model(id.to_string(), name.clone()).unwrap();

            (id, name.to_string())
        } else {
            return Err("**ERROR: No model name, attachment or url provided**".into());
        }
    };

    // Get user id
    let user_id = ctx.author().id.to_string();

    // Create the user
    add_user(user_id.clone()).unwrap();

    // Set the model as loaded
    user_set_loaded(user_id, id.to_string()).unwrap();

    status.edit(ctx, poise::CreateReply {
        content: Some(format!("Updated user model to `{}`", name)),
        ..Default::default()
    }).await?;

    Ok(())
}


/// List available models
#[poise::command(prefix_command, slash_command)]
pub async fn list(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let response = ctx.say(format!("Searching for models in directory `{}`", MODEL_DIR)).await?;

    match read_dir(MODEL_DIR) {
        Ok(entries) => {
            let mut models = Vec::new();

            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(id) = entry.file_name().into_string() {
                        if let Ok(name) = model_get_name(id.parse().unwrap()) {
                            if let Ok(metadata) = entry.metadata() {
                                let line = format!(
                                    "**Name:**\t`{}`\t**Size:**\t`{}`",
                                    name.trim_end_matches(".model"),
                                    pretty_bytes(metadata.len())
                                );
                                models.push(line);
                            }
                        }
                    }
                }
            }

            // Edit with list of models
            response.edit(ctx, poise::CreateReply {
                content: Some(format!("## Available models:\n- {}", models.join("\n- "))),
                ..Default::default()
            }).await?;
        }
        Err(err) => {
            // Edit with error message
            response.edit(ctx, poise::CreateReply {
                content: Some(format!("**ERROR: Failed to list models**\n**ERROR:** `{}`", err)),
                ..Default::default()
            }).await?;
        }
    }

    Ok(())
}

// See https://github.com/krypt0nn/markov-chains/blob/master/src/cli/model.rs

/// Get info of currently loaded model
#[poise::command(prefix_command, slash_command)]
pub async fn info(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let Ok(loaded) = user_get_loaded(ctx.author().id.to_string()) else {
        return Err("**ERROR: No model loaded**".into());
    };

    let model = postcard::from_bytes::<Model>(&std::fs::read(format!("{}/{}", MODEL_DIR, loaded))?)?;

    let formatted_headers = model.headers().iter()
        .map(|(key, value)| format!("- **{}:**\t`{}`", key, value))
        .collect::<Vec<String>>()
        .join("\n");

    let chains = (
        model.transitions()
            .trigrams_len()
            .map(|len| len.to_string())
            .unwrap_or(String::from("N/A")),

        model.transitions()
            .bigrams_len()
            .map(|len| len.to_string())
            .unwrap_or(String::from("N/A")),

        model.transitions()
            .unigrams_len()
    );

    let avg_paths = (
        model.transitions()
            .calc_avg_trigram_paths()
            .map(|avg| format!("{:.4}", avg))
            .unwrap_or(String::from("N/A")),

        model.transitions()
            .calc_avg_bigram_paths()
            .map(|avg| format!("{:.4}", avg))
            .unwrap_or(String::from("N/A")),

        format!("{:.4}", model.transitions().calc_avg_unigram_paths())
    );

    let variety = (
        model.transitions()
            .calc_trigram_variety()
            .map(|variety| format!("{:.4}%", variety * 100.0))
            .unwrap_or(String::from("N/A")),

        model.transitions()
            .calc_bigram_variety()
            .map(|variety| format!("{:.4}%", variety * 100.0))
            .unwrap_or(String::from("N/A")),

        format!("{:.4}%", model.transitions().calc_unigram_variety() * 100.0)
    );

    ctx.say(format!(
        "## Headers
{}
## Model Info
- **Total Tokens:**\t`{}`
- **Chains:**\t`{}`\t/\t`{}`\t/\t`{}`
- **Avg Paths:**\t`{}`\t/\t`{}`\t/\t`{}`
- **Variety:**\t`{}`\t/\t`{}`\t/\t`{}`",
        formatted_headers,
        model.tokens().len(),
        chains.0,
        chains.1,
        chains.2,
        avg_paths.0,
        avg_paths.1,
        avg_paths.2,
        variety.0,
        variety.1,
        variety.2
    )).await?;

    Ok(())
}
