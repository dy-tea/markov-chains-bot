use std::{
    path::PathBuf,
    fs::{read_dir, File},
    io::Write
};

use crate::{
    global::*,
    utils::pretty_bytes
};

use reqwest::header::CONTENT_DISPOSITION;

use chrono::Local;

use markov_chains::{
    prelude::*,
    dataset::Dataset,
};

#[poise::command(prefix_command, slash_command, subcommands("build", "fromscratch", "load", "list", "info"))]
pub async fn model(ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Build language model
#[poise::command(prefix_command, slash_command)]
pub async fn build(ctx: Context<'_>,
    #[description = "Path to dataset bundle"] dataset: PathBuf,
    #[description = "Build bigrams transitions table"] bigrams: bool,
    #[description = "Build trigrams transitions table"] trigrams: bool,
    #[description = "Header to add to the model"] header: Vec<String>,
    #[description = "Path to the model output"] output: PathBuf,
) -> Result<(), Error> {
    ctx.say("**UNIMPLEMENTED**").await?;
    /*
    println!("Reading dataset bundle...");

    let messages = postcard::from_bytes::<Dataset>(&std::fs::read(dataset)?)?;

    println!("Building model...");

    let mut model = Model::build(messages, bigrams, trigrams);

    for header in header {
        if let Some((key, value)) = header.split_once('=') {
            model = model.with_header(key, value);
        }
    }

    println!("Storing model...");

    std::fs::write(output, postcard::to_allocvec(&model)?)?;

    println!("Done");
    */
    Ok(())
}

/// Build language model from plain messages files
#[poise::command(prefix_command, slash_command)]
pub async fn fromscratch(
    ctx: Context<'_>,
    #[description = "Link to the plain messages file"] url: String,
    #[description = "Build bigrams transitions table"] bigrams: bool,
    #[description = "Build trigrams transitions table"] trigrams: bool,
    #[description = "Model name (overwrites file name if provided)"] name: Option<String>,
    #[description = "Model description"] description: Option<String>,
) -> Result<(), Error> {
    let status = ctx.say(format!("Attempting to download url {}", url)).await?;

    let response = reqwest::Client::new().get(url.clone()).send().await;

    let mut model_name = name.clone();

    match response {
        Ok(response) => {
            // Got response, update the user
            status.edit(ctx, poise::CreateReply {
                content: Some(format!("Download started for url {}", url)),
                ..Default::default()
            }).await?;

            // Get filename from content disposition header
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

            // Create the text file
            let mut destination = File::create(&name)?;
            let content = response.bytes().await?;
            destination.write_all(&content)?;

            // Set the model name
            model_name = if model_name.is_some() {
                model_name
            } else {
                Some(name.trim_end_matches(".txt").to_string())
            };

            // Set the download as completed
            status.edit(ctx, poise::CreateReply {
                content: Some(format!("Download completed for url {}", url)),
                ..Default::default()
            }).await?;
        },
        Err(err) => {
            return Err(format!("**ERROR: Invalid URL** {}\n**ERROR:** `{}`", url, err).into());
        },
    }

    if let Some(name) = model_name {
        let status = ctx.say(format!("Attempting to build model from file {}.txt", name)).await?;

        // Parse messages from input file
        let messages = Messages::parse_from_messages(format!("{}.txt", name))?;

        // Parse tokens from messages
        let tokens = Tokens::parse_from_messages(&messages);

        // Tokenize messages
        let tokenized_messages = TokenizedMessages::tokenize_message(&messages, &tokens)?;

        // Create the dataset
        let dataset = Dataset::default()
            .with_messages(tokenized_messages, 1)
            .with_tokens(tokens);

        // Build the model
        let mut model = Model::build(dataset, bigrams, trigrams)
            .with_header("name", name.clone())
            .with_header("created_at", Local::now().to_string());

        // Add optional description
        if let Some(description) = description {
            model = model.with_header("description", description);
        }

        // Store the model
        std::fs::write(format!("{}/{}.model", MODEL_DIR, name), postcard::to_allocvec(&model)?)?;

        // Set the model as completed
        status.edit(ctx, poise::CreateReply {
            content: Some(format!("Successfully build model `{}`", name)),
            ..Default::default()
        }).await?;

        // Delete the text file
        std::fs::remove_file(format!("{}.txt", name))?;
    }

    Ok(())
}

/// Load language model
#[poise::command(prefix_command, slash_command)]
pub async fn load(
    ctx: Context<'_>,
    #[description = "Name of model to load (e.g. kleden4)"] name: Option<String>,
    #[description = "Link to the model"] url: Option<String>,
) -> Result<(), Error> {
    let status = ctx.say("Attempting to load model").await?;

    // If name is passed, use it as model name
    let mut model_name: Option<String> = if let Some(name) = name {
        if name.ends_with(".model") {
            Some(name.trim_end_matches(".model").to_string())
        } else {
            Some(name)
        }
    } else {
        None
    };

    // If url is passed, download the model from that url
    if let Some(url) = url {
        let status = ctx.say(format!("Attempting to download model from {}", url)).await?;

        let response = reqwest::Client::new().get(url.clone()).send().await;

        match response {
            Ok(response) => {
                // Got response, update the user
                status.edit(ctx, poise::CreateReply {
                    content: Some(format!("Download started for url {}", url)),
                    ..Default::default()
                }).await?;

                // Get filename from content disposition header
                let mut name = response
                    .headers()
                    .get(CONTENT_DISPOSITION)
                    .and_then(|header| header.to_str().ok())
                    .and_then(|header_str| {
                        header_str
                            .split(';')
                            .find_map(|part| {
                                if part.trim().starts_with("filename=") {
                                    part.trim()
                                        .strip_prefix("filename=")
                                        .map(|s| s.trim_matches('"').to_string())
                                } else {
                                    None
                                }
                            })
                    })
                    .unwrap_or_else(|| {
                        let now = Local::now();
                        format!("{}", now.format("%Y-%m-%d-%H-%M-%S"))
                    });

                // Remove .model extension if present
                if name.ends_with(".model") {
                    name = name.trim_end_matches(".model").to_string();
                }

                // Create the file in the model directory
                let mut destination = File::create(&format!("{}/{}.model", MODEL_DIR, name))?;
                let content = response.bytes().await?;
                destination.write_all(&content)?;

                // Set the model name
                model_name = Some(name);

                // Set the download as completed
                status.edit(ctx, poise::CreateReply {
                    content: Some(format!("Download completed for url {}", url)),
                    ..Default::default()
                }).await?;
            },
            Err(err) => {
                return Err(format!("**ERROR: Invalid URL** {}\n**ERROR:** `{}`", url, err).into());
            },
        }
    }

    // Now either name or url should have set model_name
    if let Some(name) = model_name {
        // If name ends with .model, remove it
        let name = if name.ends_with(".model") {
            name.trim_end_matches(".model").to_string()
        } else {
            name
        };

        // Update loading status
        status.edit(ctx, poise::CreateReply {
            content: Some(format!("Attempting to load model `{}`", name)),
            ..Default::default()
        }).await?;

        // Load model from file
        let model_data = match std::fs::read(format!("{}/{}.model", MODEL_DIR, name)) {
            Ok(model) => match postcard::from_bytes::<Model>(&model) {
                Ok(model) => model,
                Err(err) => {
                    return Err(format!("**ERROR: Failed to load model** `{}`\n**ERROR:** `{}`", name, err).into());
                }
            }
            Err(err) => {
                return Err(format!("**ERROR: Failed to load model** `{}`\n**ERROR:** `{}`", name, err).into());
            }
        };

        // Update current model
        let mut model = ctx.data().model.lock().await;
        *model = model_data.clone();

        // Update current model name
        let mut model_name = ctx.data().model_name.lock().await;
        *model_name = name.clone();

        // Edit the message with loaded status
        status.edit(ctx, poise::CreateReply {
            content: Some(format!("Successfully loaded model `{}` ", name)),
            ..Default::default()
        }).await?;

        return Ok(());
    }

    // No model name or url was provided
    return Err("Please provide either a model name or a url".into());
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
                    if let Ok(name) = entry.file_name().into_string() {
                        if name.ends_with(".model") {
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
    let model = ctx.data().model.lock().await;
    let headers = model.headers().clone();

    let formatted_headers = headers.iter()
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
