use super::{VFunction, Weights};
use game::Board;

#[derive(Debug, Clone, Default)]
pub struct NTupleSmallWeights {
    pub corner: [f32; 16],
    pub edge: [f32; 16],
    pub center: [f32; 16],
}

impl Weights for NTupleSmallWeights {
    fn optimized() -> NTupleSmallWeights {
        NTupleSmallWeights {
            corner: [
                191.37343, 78.2322, 54.284393, 3.0459914, -44.783875, -25.07536, 18.911865,
                20.963875, -31.210142, -4.3872957, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            edge: [
                162.0052, 113.46235, 89.72123, 38.008884, -9.765412, 11.449189, 55.578365,
                57.867737, 6.692998, -2.3696995, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            center: [
                -29.262695, 35.25711, 35.45671, 34.975533, 35.016354, 36.523926, 36.665627,
                36.90312, 37.903225, 2.0175886, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct NTupleSmall {
    weights: NTupleSmallWeights,
}

impl VFunction for NTupleSmall {
    type Weights = NTupleSmallWeights;

    fn new(weights: NTupleSmallWeights) -> NTupleSmall {
        NTupleSmall { weights }
    }

    fn eval(&self, state: &Board) -> f32 {
        let mut eval = 0.;

        for i in &[0, 3] {
            let row = state.row_at(*i);

            eval += self.weights.corner[row.tile_at(0) as usize];
            eval += self.weights.edge[row.tile_at(1) as usize];
            eval += self.weights.edge[row.tile_at(2) as usize];
            eval += self.weights.corner[row.tile_at(3) as usize];
        }

        for i in &[1, 2] {
            let row = state.row_at(*i);

            eval += self.weights.edge[row.tile_at(0) as usize];
            eval += self.weights.center[row.tile_at(1) as usize];
            eval += self.weights.center[row.tile_at(2) as usize];
            eval += self.weights.edge[row.tile_at(3) as usize];
        }

        eval
    }

    fn learn(&mut self, position: &Board, delta: &f32) {
        let adjusted_delta = delta / 16.;

        for i in &[0, 3] {
            let row = position.row_at(*i);
            self.weights.corner[row.tile_at(0) as usize] += adjusted_delta;
            self.weights.edge[row.tile_at(1) as usize] += adjusted_delta;
            self.weights.edge[row.tile_at(2) as usize] += adjusted_delta;
            self.weights.corner[row.tile_at(3) as usize] += adjusted_delta;
        }

        for i in &[0, 3] {
            let row = position.row_at(*i);
            self.weights.edge[row.tile_at(0) as usize] += adjusted_delta;
            self.weights.center[row.tile_at(1) as usize] += adjusted_delta;
            self.weights.center[row.tile_at(2) as usize] += adjusted_delta;
            self.weights.edge[row.tile_at(3) as usize] += adjusted_delta;
        }
    }

    fn into_weights(self) -> NTupleSmallWeights {
        self.weights
    }
}
