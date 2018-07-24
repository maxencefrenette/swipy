pub struct Config {
    pub score_per_empty: f32,
    pub continuation_bonus: f32,
}

pub const OPTIMIZED_CONFIG: Config = Config {
    score_per_empty: 200.,
    continuation_bonus: 100.,
};
