pub use crate::global::*;

/// Send a query to the currently loaded model
#[poise::command(prefix_command, slash_command, broadcast_typing)]
pub async fn query(
    ctx: Context<'_>,
    #[description = "Starting query to run the current model off"] query: String,
) -> Result<(), Error> {
    // Get the currently loaded model
    let model = ctx.data().model.lock().await.clone();

    // Get the model name
    let model_name = ctx.data().model_name.lock().await.clone();

    // Get the current parameters
    let params = ctx.data().params.lock().await.clone();

    // Display the current query
    let query_message = format!("## Created Query\n- **Model:**\t`{}`\n- **Query:**\t `{}`\n- **Status:**\t",  model_name, query);
    let query_reply = ctx.say(format!("{}`Querying...`", query_message)).await?;

    // Generate the current query
    let message_start = query.clone();
    let query = query.split_whitespace()
        .filter(|word| !word.is_empty())
        .map(|word| word.to_lowercase())
        .map(|word| model.tokens().find_token(word))
        .collect::<Option<Vec<_>>>();

    let Some(query) = query else {
        ctx.say("**ERROR: Query not in dataset**").await?;
        return Ok(());
    };

    if query.is_empty() {
        ctx.say("**ERROR: Query cannot be empty**").await?;
        return Ok(());
    }

    let generated = ctx.say("Generating...").await?;
    let mut message = message_start;
    for token in model.generate(query, &params) {
        match token {
            Ok(token) => {
                let Some(word) = model.tokens().find_word(token) else {
                    ctx.say(format!("**ERROR: Failed to find word for token** `{}`", token)).await?;
                    break;
                };

                message = format!("{} {}", message.clone(), word);
                generated.edit(ctx, poise::CreateReply {
                    content: Some(message.clone()),
                    ..Default::default()
                }).await?;
            }

            Err(err) => {
                ctx.say(format!("**ERROR: Failed to generate** `{}`", err)).await?;
                break;
            }
        }
    }

    // Edit message to show it's completed
    query_reply.edit(ctx, poise::CreateReply {
        content: Some(format!("{}`Completed`", query_message)),
        ..Default::default()
    }).await?;

    Ok(())
}
