use super::{VFunction, Weights};
use game::Board;

lazy_static! {
    static ref OPTIMIZED: LegacyWeights =
        serde_json::from_slice(include_bytes!("../../../networks/legacy.json"))
            .expect("valid legacy weights");
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LegacyWeights {
    pub outer_pos_bonus: [f32; 16],
    pub inner_pos_bonus: [f32; 16],
}

impl Weights for LegacyWeights {
    fn optimized() -> LegacyWeights {
        OPTIMIZED.clone()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Legacy {
    weights: LegacyWeights,
}

impl VFunction for Legacy {
    type Weights = LegacyWeights;

    fn new(weights: LegacyWeights) -> Legacy {
        Legacy { weights }
    }

    fn eval(&self, state: &Board) -> f32 {
        let mut eval = 0.;

        for i in 0..4 {
            let row = state.row_at(i);
            let column = state.column_at(i);

            eval += self.weights.outer_pos_bonus[row.tile_at(0) as usize];
            eval += self.weights.inner_pos_bonus[row.tile_at(1) as usize];
            eval += self.weights.inner_pos_bonus[row.tile_at(2) as usize];
            eval += self.weights.outer_pos_bonus[row.tile_at(3) as usize];
            eval += self.weights.outer_pos_bonus[column.tile_at(0) as usize];
            eval += self.weights.inner_pos_bonus[column.tile_at(1) as usize];
            eval += self.weights.inner_pos_bonus[column.tile_at(2) as usize];
            eval += self.weights.outer_pos_bonus[column.tile_at(3) as usize];
        }

        eval
    }

    fn learn(&mut self, position: &Board, delta: &f32) {
        let adjusted_delta = delta / 32.;

        for i in 0..4 {
            let row = position.row_at(i);
            let column = position.column_at(i);

            self.weights.outer_pos_bonus[row.tile_at(0) as usize] += adjusted_delta;
            self.weights.inner_pos_bonus[row.tile_at(1) as usize] += adjusted_delta;
            self.weights.inner_pos_bonus[row.tile_at(2) as usize] += adjusted_delta;
            self.weights.outer_pos_bonus[row.tile_at(3) as usize] += adjusted_delta;
            self.weights.outer_pos_bonus[column.tile_at(0) as usize] += adjusted_delta;
            self.weights.inner_pos_bonus[column.tile_at(1) as usize] += adjusted_delta;
            self.weights.inner_pos_bonus[column.tile_at(2) as usize] += adjusted_delta;
            self.weights.outer_pos_bonus[column.tile_at(3) as usize] += adjusted_delta;
        }
    }

    fn into_weights(self) -> LegacyWeights {
        self.weights
    }
}
