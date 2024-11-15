use std::path::PathBuf;

pub use crate::global::*;

use markov_chains::{
    messages::Messages,
    tokens::Tokens,
    tokenized_messages::TokenizedMessages
};

use super::search_files;

#[poise::command(prefix_command, slash_command, subcommands("parse", "merge", "tokenize"))]
pub async fn messages(
    ctx: Context<'_>,
) -> Result<(), Error> {
    Ok(())
}

/// Parse messages from a file to a bundle
#[poise::command(prefix_command, slash_command)]
pub async fn parse(
    ctx: Context<'_>,
    #[description = "Paths to the messages list"] path: Vec<PathBuf>,
    #[description = "Path to the bundle output"] output: PathBuf
) -> Result<(), Error> {
    /*
    let mut messages = Messages::default();

    println!("Parsing messages...");

    for path in search_files(path) {
        println!("Parsing {:?}...", path);

        messages = messages.merge(Messages::parse_from_messages(path)?);
    }

    println!("Storing messages bundle...");

    std::fs::write(output, postcard::to_allocvec(&messages)?)?;

    println!("Done");
    */

    Ok(())
}

/// Merge different messages bundles into a single file
#[poise::command(prefix_command, slash_command)]
pub async fn merge(
    ctx: Context<'_>,
    #[description = "Paths to the messages bundles"] path: Vec<PathBuf>,
    #[description = "Path to the merged messages bundle"] output: PathBuf
) -> Result<(), Error> {
    /*
    let mut messages = Messages::default();

    println!("Reading messages bundles...");

    for path in search_files(path) {
        println!("Reading {:?}...", path);

        let bundle = postcard::from_bytes::<Messages>(&std::fs::read(path)?)?;

        messages = messages.merge(bundle);
    }

    println!("Storing merged messages bundle...");

    std::fs::write(output, postcard::to_allocvec(&messages)?)?;

    println!("Done");
    */

    Ok(())
}

/// Tokenize messages bundle
#[poise::command(prefix_command, slash_command)]
pub async fn tokenize(
    ctx: Context<'_>,
    #[description = "Path to the messages bundle"] messages: PathBuf,
    #[description = "Path to the tokens bundle"] tokens: PathBuf,
    #[description = "Path to the tokenized messages bundle"] output: PathBuf,
) -> Result<(), Error> {
    /*
    println!("Reading messages bundle...");

    let messages = postcard::from_bytes::<Messages>(&std::fs::read(messages)?)?;

    println!("Reading tokens bundle...");

    let tokens = postcard::from_bytes::<Tokens>(&std::fs::read(tokens)?)?;

    println!("Tokenizing messages...");

    let tokenized = TokenizedMessages::tokenize_message(&messages, &tokens)?;

    println!("Storing tokenized messages bundle...");

    std::fs::write(output, postcard::to_allocvec(&tokenized)?)?;

    println!("Done");
    */
    Ok(())
}
