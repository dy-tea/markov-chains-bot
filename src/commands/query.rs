pub use crate::global::*;

/// Send a query to the currently loaded model
#[poise::command(prefix_command, slash_command)]
pub async fn query(
    ctx: Context<'_>,
    #[description = "Starting query to run the current model off"] query: String,
) -> Result<(), Error> {
    Ok(())
}
