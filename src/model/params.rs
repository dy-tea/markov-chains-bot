pub struct GenerationParams {
    pub temperature: f64,
    pub temperature_alpha: f64,
    pub repeat_penalty: f64,
    pub repeat_penalty_window: usize,
    pub k_normal: f64,
    pub min_len: usize,
    pub max_len: usize,
    pub no_bigrams: bool,
    pub no_trigrams: bool,
}
