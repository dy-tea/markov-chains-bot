use markov_chains::prelude::GenerationParams;

use crate::{
    global::*,
    db::*
};

fn format_params(params: &GenerationParams) -> String {
    format!("**Params:**
- **temperature**\t`{}`
- **temperature_alpha:**\t`{}`
- **repeat_penalty:**\t`{}`
- **repeat_penalty_window:**\t`{}`
- **k_normal:**\t`{}`
- **min_len:**\t`{}`
- **max_len:**\t`{}`
- **no_bigrams:**\t`{}`
- **no_trigrams:**\t`{}`",
        params.temperature,
        params.temperature_alpha,
        params.repeat_penalty,
        params.repeat_penalty_window,
        params.k_normal,
        params.min_len,
        params.max_len,
        params.no_bigrams,
        params.no_trigrams
    )
}

#[poise::command(prefix_command, slash_command, subcommand_required, subcommands("set", "reset", "show"))]
pub async fn params(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Update the model parameters
#[poise::command(prefix_command, slash_command)]
pub async fn set(
    ctx: Context<'_>,
    #[description = "Probability to keep the most probable token"] temperature: Option<f64>,
    #[description = "Probability multiplier to skip the most probable token"] temperature_alpha: Option<f64>,
    #[description = "Reverse probability to skip repeated token"] repeat_penalty: Option<f64>,
    #[description = "Size of window which calculates repeats number"] repeat_penalty_window: Option<usize>,
    #[description = "Percent of tokens to keep from the normal distribution"] k_normal: Option<f64>,
    #[description = "Minimum length of the generated text"] min_len: Option<usize>,
    #[description = "Maximum length of the generated text"] max_len: Option<usize>,
    #[description = "Do not use bigrams for text generation"] no_bigrams: Option<bool>,
    #[description = "Do not use trigrams for text generation"] no_trigrams: Option<bool>,
) -> Result<(), Error> {
    let author_id = ctx.author().id.to_string();

    let params = user_get_params(author_id.clone()).unwrap();

    user_set_params(author_id.clone(), GenerationParams {
        temperature: temperature.unwrap_or(params.temperature),
        temperature_alpha: temperature_alpha.unwrap_or(params.temperature_alpha),
        repeat_penalty: repeat_penalty.unwrap_or(params.repeat_penalty),
        repeat_penalty_window: repeat_penalty_window.unwrap_or(params.repeat_penalty_window),
        k_normal: k_normal.unwrap_or(params.k_normal),
        min_len: min_len.unwrap_or(params.min_len),
        max_len: max_len.unwrap_or(params.max_len),
        no_bigrams: no_bigrams.unwrap_or(params.no_bigrams),
        no_trigrams: no_trigrams.unwrap_or(params.no_trigrams),
    }).unwrap();

    let new_params = user_get_params(author_id).unwrap();

    ctx.say(format!("## Params have been updated\n{}", format_params(&new_params))).await?;

    Ok(())
}

/// Show current model parameters
#[poise::command(prefix_command, slash_command)]
pub async fn show(ctx: Context<'_>) -> Result<(), Error>  {
    let params = user_get_params(ctx.author().id.to_string()).unwrap();

    ctx.say(format_params(&params)).await?;

    Ok(())
}

/// Reset model parameters to default values
#[poise::command(prefix_command, slash_command)]
pub async fn reset(ctx: Context<'_>) -> Result<(), Error>  {
    user_set_params(ctx.author().id.to_string(), GenerationParams::default()).unwrap();

    ctx.say("**Params have been reset**").await?;

    Ok(())
}
