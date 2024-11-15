use markov_chains::prelude::GenerationParams;

pub use crate::global::*;

/// Update the model parameters
#[poise::command(prefix_command, slash_command)]
pub async fn set_params(
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
    let mut temp = ctx.data().temp.lock().await;
    let mut data = match temp.get("params").unwrap() {
        GlobalData::Params(data) => data.clone(),
        _ => {
            ctx.say("**ERROR: No parameters loaded**").await?;
            return Ok(());
        }
    };

    data = GenerationParams {
        temperature: temperature.unwrap_or(data.temperature),
        temperature_alpha: temperature_alpha.unwrap_or(data.temperature_alpha),
        repeat_penalty: repeat_penalty.unwrap_or(data.repeat_penalty),
        repeat_penalty_window: repeat_penalty_window.unwrap_or(data.repeat_penalty_window),
        k_normal: k_normal.unwrap_or(data.k_normal),
        min_len: min_len.unwrap_or(data.min_len),
        max_len: max_len.unwrap_or(data.max_len),
        no_bigrams: no_bigrams.unwrap_or(data.no_bigrams),
        no_trigrams: no_trigrams.unwrap_or(data.no_trigrams),
    };

    temp.remove("params").unwrap();
    temp.insert("params", GlobalData::Params(data));

    Ok(())
}

/// Reset model parameters to default values
#[poise::command(prefix_command, slash_command)]
pub async fn reset_params(ctx: Context<'_>) -> Result<(), Error>  {
    // Get temporary data
    let mut temp = ctx.data().temp.lock().await;

    temp.remove("params").unwrap();
    temp.insert("params", GlobalData::Params(GenerationParams::default()));

    Ok(())
}
