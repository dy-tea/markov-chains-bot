pub use crate::global::*;

/// Update the model parameters
#[poise::command(prefix_command, slash_command)]
pub async fn set_params(
    ctx: Context<'_>,
    #[description = "Probability to keep the most probable token

If `random_seed > temperature * temperature_alpha^[token number]`,
then the most probable token is skipped.

Lower temperature generates more random text.

`random_seed` is a random number from 0.0 to 1.0."] temperature: Option<f64>,
    #[description = "Probability multiplier to skip the most probable token"] temperature_alpha: Option<f64>,
    #[description = "Reverse probability to skip repeated token

If `random_seed > repeat_penalty^[repeats number]`,
then the repeated token is skipped.

Lower penalty skips repeated tokens more aggressively.

`random_seed` is a random number from 0.0 to 1.0."] repeat_penalty: Option<f64>,
    #[description = "Size of window which calculates repeats number"] repeat_penalty_window: Option<usize>,
    #[description = "Percent of tokens to keep from the normal distribution

Other tokens will be removed equally from the beginning
(least probable) and end (most probable).

Lower value will generate more \"bot-looking\" (weird) text."] k_normal: Option<f64>,
    #[description = "Minimum length of the generated text"] min_len: Option<usize>,
    #[description = "Maximum length of the generated text"] max_len: Option<usize>,
    #[description = "Do not use bigrams for text generation"] no_bigrams: Option<bool>,
    #[description = "Do not use trigrams for text generation"] no_trigrams: Option<bool>,
) -> Result<(), Error> {
    let mut data = ctx.data().params.lock().unwrap();

    /*
        Default Values:

        temperature: 0.85,
        temperature_alpha: 1.0,
        repeat_penalty: 0.7,
        repeat_penalty_window: 10,
        k_normal: 0.95,
        min_len: 1,
        max_len: 150,
        no_bigrams: false,
        no_trigrams: false
    */

    data.temperature = temperature.unwrap_or(0.85);
    data.temperature_alpha = temperature_alpha.unwrap_or(1.0);
    data.repeat_penalty = repeat_penalty.unwrap_or(0.7);
    data.repeat_penalty_window = repeat_penalty_window.unwrap_or(10);
    data.k_normal = k_normal.unwrap_or(0.95);
    data.min_len = min_len.unwrap_or(1);
    data.max_len = max_len.unwrap_or(150);
    data.no_bigrams = no_bigrams.unwrap_or(false);
    data.no_trigrams = no_trigrams.unwrap_or(false);

    Ok(())
}
