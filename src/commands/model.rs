use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub use crate::global::*;

use markov_chains::prelude::{
    Messages,
    Tokens,
    TokenizedMessages,
    Dataset,
    GenerationParams,
    Model
};

use super::search_files;

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
    #[description = "Name of model to load (e.g. `kleden4`)"] name: Option<String>,
    #[description = "Link to the model"] model: Option<PathBuf>,
) -> Result<(), Error> {
    if let Some(name) = name {
        // Load model from file
        let model = postcard::from_bytes::<Model>(
            &std::fs::read(
                format!("{}/{}.model", MODEL_DIR, name)
            )?
        )?;

        // Update current model
        {
            let mut data = ctx.data().model.lock().unwrap();
            *data = model;
        }

        // Update current model name
        {
            let mut data = ctx.data().model_name.lock().unwrap();
            *data = name.clone();
        }

        ctx.say(format!("Model **{}** loaded successfully", name)).await?;
    } else if let Some(model) = model {
        ctx.say("TODO: Load from url").await?;
    } else {
        ctx.say("You need to provide either a model name or a model file").await?;
    }

    /*
    println!("Reading model...");

    let model = postcard::from_bytes::<Model>(&std::fs::read(model)?)?;

    println!("Starting model...");

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    let chains = (
        model.transitions.trigrams_len()
            .map(|len| len.to_string())
            .unwrap_or(String::from("N/A")),

        model.transitions.bigrams_len()
            .map(|len| len.to_string())
            .unwrap_or(String::from("N/A")),

        model.transitions.unigrams_len()
    );

    let avg_paths = (
        model.transitions.calc_avg_trigram_paths()
            .map(|avg| format!("{:.4}", avg))
            .unwrap_or(String::from("N/A")),

        model.transitions.calc_avg_bigram_paths()
            .map(|avg| format!("{:.4}", avg))
            .unwrap_or(String::from("N/A")),

        format!("{:.4}", model.transitions.calc_avg_unigram_paths())
    );

    let variety = (
        model.transitions.calc_trigram_variety()
            .map(|variety| format!("{:.4}%", variety * 100.0))
            .unwrap_or(String::from("N/A")),

        model.transitions.calc_bigram_variety()
            .map(|variety| format!("{:.4}%", variety * 100.0))
            .unwrap_or(String::from("N/A")),

        format!("{:.4}%", model.transitions.calc_unigram_variety() * 100.0)
    );

    let model_name = model.headers()
        .get("name")
        .map(|name| name.as_str())
        .unwrap_or("model");

    println!();
    println!("  Model loaded:");
    println!();
    println!("    Total tokens  :  {}", model.tokens.len());
    println!("    Chains        :  {} / {} / {}", chains.0, chains.1, chains.2);
    println!("    Avg paths     :  {} / {} / {}", avg_paths.0, avg_paths.1, avg_paths.2);
    println!("    Variety       :  {} / {} / {}", variety.0, variety.1, variety.2);

    if !model.headers().is_empty() {
        println!();
        println!("  Headers:");
        println!();

        let max_len = model.headers()
            .keys()
            .map(|key| key.len())
            .max()
            .unwrap_or(0);

        for (key, value) in model.headers() {
            let offset = " ".repeat(max_len - key.len());

            println!("    [{key}]{offset} : {value}");
        }
    }

    println!();

    loop {
        let mut request = String::new();

        stdout.write_all(b"> ")?;
        stdout.flush()?;

        stdin.read_line(&mut request)?;

        let request = request.split_whitespace()
            .filter(|word| !word.is_empty())
            .map(|word| word.to_lowercase())
            .map(|word| model.tokens.find_token(word))
            .collect::<Option<Vec<_>>>();

        let Some(request) = request else {
            continue;
        };

        if request.is_empty() {
            continue;
        }

        stdout.write_all(format!("\n  {model_name}: ").as_bytes())?;
        stdout.flush()?;

        for token in &request {
            stdout.write_all(model.tokens.find_word(*token).unwrap().as_bytes())?;
            stdout.write_all(b" ")?;
            stdout.flush()?;
        }

        for token in model.generate(request, params) {
            match token {
                Ok(token) => {
                    let Some(word) = model.tokens.find_word(token) else {
                        print!("\n\n  Failed to find word for token: {token}");

                        break;
                    };

                    stdout.write_all(word.as_bytes())?;
                    stdout.write_all(b" ")?;
                    stdout.flush()?;
                }

                Err(err) => {
                    print!("\n\n  Failed to generate: {err}");

                    break;
                }
            }
        }

        stdout.write_all(b"\n\n")?;
        stdout.flush()?;
    }
    */

    Ok(())
}
