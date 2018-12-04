use super::{VFunction, Weights};
use game::Board;

#[derive(Debug, Clone, Default)]
pub struct LegacyWeights {
    pub outer_pos_bonus: [f32; 16],
    pub inner_pos_bonus: [f32; 16],
}

impl Weights for LegacyWeights {
    fn optimized() -> LegacyWeights {
        LegacyWeights {
            outer_pos_bonus: [
                187.51797, 146.52249, 118.49475, 57.08678, 19.882929, 33.229523, -6.8986917,
                -130.95418, -349.45972, -388.40686, -0.4167655, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            inner_pos_bonus: [
                162.26865, 136.7995, 109.440506, 94.70556, 73.923134, 41.334373, -11.293593,
                -140.11522, -363.43854, -411.76, -5.027099, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
        }
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

            eval += row.score() / 2.;
            eval += column.score() / 2.;

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
        let adjusted_delta = delta / 8.;

        for i in 0..4 {
            let row = position.row_at(i);
            self.weights.outer_pos_bonus[row.tile_at(0) as usize] += adjusted_delta;
            self.weights.inner_pos_bonus[row.tile_at(1) as usize] += adjusted_delta;
            self.weights.inner_pos_bonus[row.tile_at(2) as usize] += adjusted_delta;
            self.weights.outer_pos_bonus[row.tile_at(3) as usize] += adjusted_delta;

            let column = position.column_at(i);
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
