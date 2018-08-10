#[derive(Debug)]
pub struct Config {
    pub score_per_empty: f32,
    pub continuation_bonus: f32,
}

pub const OPTIMIZED_CONFIG: Config = Config {
    score_per_empty: 382.54465,
    continuation_bonus: 65.536255,
};

impl Config {
    pub fn from_vec(params: Vec<f32>) -> Config {
        Config {
            score_per_empty: params[0],
            continuation_bonus: params[1],
        }
    }

    pub fn to_vec(&self) -> Vec<f32> {
        vec![self.score_per_empty, self.continuation_bonus]
    }
}
