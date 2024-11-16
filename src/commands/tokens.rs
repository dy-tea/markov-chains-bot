use std::path::PathBuf;

pub use crate::global::*;

#[poise::command(prefix_command, slash_command, subcommands("parse", "merge"))]
pub async fn tokens(
    ctx: Context<'_>,
) -> Result<(), Error> {
    Ok(())
}

/// Parse tokens from a messages bundle
#[poise::command(prefix_command, slash_command)]
pub async fn parse(
    ctx: Context<'_>,
    #[description = "Path to the messages bundle"] path: Vec<PathBuf>,
    #[description = "Path to the tokens output"] output: PathBuf,
) -> Result<(), Error> {
    ctx.say("**UNIMPLEMENTED**").await?;
    /*
    println!("Reading messages bundles...");

    let mut messages = Messages::default();

    for path in search_files(path) {
        println!("Reading {:?}...", path);

        messages = messages.merge(postcard::from_bytes::<Messages>(&std::fs::read(path)?)?);
    }

    println!("Generating tokens...");

    let tokens = Tokens::parse_from_messages(&messages);

    println!("Storing tokens bundle...");

    std::fs::write(output, postcard::to_allocvec(&tokens)?)?;

    println!("Done");
    */

    Ok(())
}

/// Merge tokens bundles
#[poise::command(prefix_command, slash_command)]
pub async fn merge(
    ctx: Context<'_>,
    #[description = "Path to the tokens bundle"] path: Vec<PathBuf>,
    #[description = "Path to the merged tokens output"] output: PathBuf,
) -> Result<(), Error> {
    ctx.say("**UNIMPLEMENTED**").await?;
    /*
    println!("Reading tokens bundles...");

    let mut tokens = Tokens::default();

    for path in search_files(path) {
        println!("Reading {:?}...", path);

        tokens = tokens.merge(postcard::from_bytes::<Tokens>(&std::fs::read(path)?)?);
    }

    println!("Storing merged tokens bundle...");

    std::fs::write(output, postcard::to_allocvec(&tokens)?)?;

    println!("Done");
    */

    Ok(())
}
