use std::path::PathBuf;

pub use crate::global::*;

use markov_chains::prelude::*;

#[poise::command(prefix_command, slash_command, subcommands("build", "fromscratch", "load"))]
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
    #[description = "Paths to the plain messages file"] paths: Vec<PathBuf>,
    #[description = "Build bigrams transitions table"] bigrams: bool,
    #[description = "Build trigrams transitions table"] trigrams: bool,
    #[description = "Header to add to the model"] header: Vec<String>,
    #[description = "Path to the model output"] output: PathBuf,
) -> Result<(), Error> {
    ctx.say("**UNIMPLEMENTED**").await?;
    /*
    println!("Parsing messages...");

    let mut messages = Messages::default();

    for path in search_files(paths) {
        println!("Parsing {:?}...", path);

        let parsed = Messages::parse_from_messages(path)?;

        messages = messages.merge(parsed);
    }

    println!("Generating tokens...");

    let tokens = Tokens::parse_from_messages(&messages);

    println!("Tokenizing messages...");

    let tokenized_messages = TokenizedMessages::tokenize_message(&messages, &tokens)?;

    println!("Creating dataset...");

    let dataset = Dataset::default()
        .with_messages(tokenized_messages, 1)
        .with_tokens(tokens);

    println!("Building model...");

    let mut model = Model::build(dataset, bigrams, trigrams);

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

/// Load language model
#[poise::command(prefix_command, slash_command)]
pub async fn load(
    ctx: Context<'_>,
    #[description = "Name of model to load (e.g. kleden4)"] name: Option<String>,
    #[description = "Link to the model"] url: Option<PathBuf>,
) -> Result<(), Error> {
    if let Some(name) = name {
        // Send a message with the loading status
        let status = ctx.say(format!("Attempting to load model **{}.model**", name)).await?;

        // Load model from file
        let model_data = match std::fs::read(format!("{}/{}.model", MODEL_DIR, name)) {
            Ok(model) => match postcard::from_bytes::<Model>(&model) {
                Ok(model) => model,
                Err(err) => {
                    status.edit(ctx, poise::CreateReply {
                        content: Some(format!("**ERROR: Failed to load model \"{}.model\"** `{}`", name, err)),
                        ..Default::default()
                    }).await?;
                    return Ok(());
                }
            }
            Err(err) => {
                status.edit(ctx, poise::CreateReply {
                    content: Some(format!("**ERROR: Failed to load model \"{}.model\"** `{}`", name, err)),
                    ..Default::default()
                }).await?;
                return Ok(());
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
            content: Some(format!("Model **{}** loaded successfully", name)),
            ..Default::default()
        }).await?;

        return Ok(());
    }

    if let Some(url) = url {
        ctx.say("TODO: Load from url").await?;
        return Ok(());
    }

    ctx.say("Please provide either a model name or a url").await?;
    Ok(())
}
