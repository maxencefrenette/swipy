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
                295.7805,
                202.71933,
                145.28839,
                5.6943674,
                -128.8403,
                -67.74326,
                -14.902347,
                -8.068005,
                -6.1175094,
                -0.27227518,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
            ],
            edge: [
                322.1197,
                253.4931,
                196.10011,
                56.489624,
                -15.299292,
                -7.852551,
                26.219332,
                16.572641,
                0.18946102,
                -0.058653127,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
            ],
            center: [
                24.940155, 51.352448, 51.60894, 50.790497, 113.53783, 59.896114, 41.12308,
                24.640617, 6.306972, 0.21362203, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
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

            eval += row.score();
            eval += self.weights.corner[row.tile_at(0) as usize];
            eval += self.weights.edge[row.tile_at(1) as usize];
            eval += self.weights.edge[row.tile_at(2) as usize];
            eval += self.weights.corner[row.tile_at(3) as usize];
        }

        for i in &[1, 2] {
            let row = state.row_at(*i);

            eval += row.score();
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
