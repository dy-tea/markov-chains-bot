use std::path::PathBuf;

pub use crate::global::*;

use crate::search_files;
use markov_chains::{
    dataset::Dataset,
    tokens::Tokens,
    tokenized_messages::TokenizedMessages
};

#[poise::command(prefix_command, slash_command, subcommands("create", "addmessages", "addtokens", "checkword"))]
pub async fn dataset(
    ctx: Context<'_>
) -> Result<(), Error> {
    Ok(())
}

/// Create dataset from the tokenized messages and tokens bundle
#[poise::command(prefix_command, slash_command)]
pub async fn create(
    ctx: Context<'_>,
    #[description = "Path to the messages bundle"] path: PathBuf,
    #[description = "Path to the tokens bundle"] tokens: PathBuf,
    #[description = "Messages weight in the dataset"] weight: u64,
    #[description = "Path to the dataset output"] output: PathBuf,
) -> Result<(), Error> {
    ctx.say("**UNIMPLEMENTED**").await?;
    /*
    println!("Reading tokenized messages bundle...");

    let tokenized_messages = postcard::from_bytes::<TokenizedMessages>(&std::fs::read(path)?)?;

    println!("Reading tokens bundle...");

    let tokens = postcard::from_bytes::<Tokens>(&std::fs::read(tokens)?)?;

    println!("Creating dataset...");

    let dataset = Dataset::default()
        .with_messages(tokenized_messages, weight)
        .with_tokens(tokens);

    println!("Storing dataset bundle...");

    std::fs::write(output, postcard::to_allocvec(&dataset)?)?;

    println!("Done");
     */

    Ok(())
}

/// Extend existing dataset with the tokenized messages
#[poise::command(prefix_command, slash_command)]
pub async fn addmessages(
    ctx: Context<'_>,
    #[description = "Path to the dataset bundle"] path: PathBuf,
    #[description = "Path to the messages bundle"] messages: Vec<PathBuf>,
    #[description = "Messages weight in the dataset"] weight: u64,
    #[description = "Path to the dataset output"] output: PathBuf,
) -> Result<(), Error> {
    ctx.say("**UNIMPLEMENTED**").await?;
    /*
    println!("Reading dataset bundle...");

    let mut dataset = postcard::from_bytes::<Dataset>(&std::fs::read(path)?)?;

    println!("Reading tokenized messages bundles...");

    for path in search_files(messages) {
        println!("Reading {:?}...", path);

        let tokenized_messages = postcard::from_bytes::<TokenizedMessages>(&std::fs::read(path)?)?;

        dataset = dataset.with_messages(tokenized_messages, weight);
    }

    println!("Storing dataset bundle...");

    std::fs::write(output, postcard::to_allocvec(&dataset)?)?;

    println!("Done");
 */
    Ok(())
}

/// Extend existing dataset with the tokenized messages
#[poise::command(prefix_command, slash_command)]
pub async fn addtokens(
    ctx: Context<'_>,
    #[description = "Path to the messages bundle"] path: PathBuf,
    #[description = "Path to the tokens bundle"] tokens: Vec<PathBuf>,
    #[description = "Path to the dataset output"] output: PathBuf,
) -> Result<(), Error> {
    ctx.say("**UNIMPLEMENTED**").await?;
    /*
    println!("Reading dataset bundle...");

    let mut dataset = postcard::from_bytes::<Dataset>(&std::fs::read(path)?)?;

    println!("Reading tokens bundles...");

    for path in search_files(tokens) {
        println!("Reading {:?}...", path);

        let tokens = postcard::from_bytes::<Tokens>(&std::fs::read(path)?)?;

        dataset = dataset.with_tokens(tokens);
    }

    println!("Storing dataset bundle...");

    std::fs::write(output, postcard::to_allocvec(&dataset)?)?;

    println!("Done");
    */

    Ok(())
}

/// Check the word appearance in the dataset
#[poise::command(prefix_command, slash_command)]
pub async fn checkword(
    ctx: Context<'_>,
    #[description = "Path to the dataset bundle"] path: PathBuf,
    #[description = "Word to check"] word: String,
) -> Result<(), Error> {
    ctx.say("**UNIMPLEMENTED**").await?;
    /*
    println!("Reading dataset bundle...");

    let dataset = postcard::from_bytes::<Dataset>(&std::fs::read(path)?)?;

    println!("Checking word appearance...");

    let Some(token) = dataset.tokens().find_token(&word) else {
        println!("Could not find token for word: {}", word);
        return Ok(());
    };

    let mut distinct_num = 0;
    let mut total_num = 0;
    let mut importance = 0;

    let mut total_messages = 0;

    for (message, weight) in dataset.messages() {
        for message in message.messages() {
            let num = message.iter().filter(|t| *t == &token).count() as u64;

            distinct_num += if num > 0 { 1 } else { 0 };
            total_num += num;

            importance += num * *weight;

            total_messages += 1;
        }
    }

    println!();
    println!("Distinct num: {distinct_num}");
    println!("   Total num: {total_num}");
    println!("  Importance: {importance}");
    println!("   Frequency: {:.5}%", distinct_num as f64 / total_messages as f64 * 100.0);
    */

    Ok(())
}
