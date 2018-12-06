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
                108.54316, 53.69378, 59.764202, 51.69314, 58.505787, 53.682144, 7.3442073,
                -71.988304, -198.15659, -298.21445, -20.902882, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            inner_pos_bonus: [
                125.3738, 106.51863, 115.91801, 106.5212, 83.4218, 44.721077, -8.361226,
                -114.07304, -277.46603, -345.0825, -33.7291, 0.0, 0.0, 0.0, 0.0, 0.0,
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
