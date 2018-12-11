use super::{VFunction, Weights};
use crate::game::Board;
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};

lazy_static! {
    static ref OPTIMIZED: NTupleSmallWeights =
        serde_json::from_slice(include_bytes!("../../../networks/n_tuple_small.json"))
            .expect("valid legacy weights");
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NTupleSmallWeights {
    pub corner: [f32; 16],
    pub edge: [f32; 16],
    pub center: [f32; 16],
}

impl Weights for NTupleSmallWeights {
    fn optimized() -> NTupleSmallWeights {
        OPTIMIZED.clone()
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

    fn eval(&self, state: Board) -> f32 {
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

    fn learn(&mut self, state: Board, delta: f32) {
        let adjusted_delta = delta / 16.;

        for i in &[0, 3] {
            let row = state.row_at(*i);
            self.weights.corner[row.tile_at(0) as usize] += adjusted_delta;
            self.weights.edge[row.tile_at(1) as usize] += adjusted_delta;
            self.weights.edge[row.tile_at(2) as usize] += adjusted_delta;
            self.weights.corner[row.tile_at(3) as usize] += adjusted_delta;
        }

        for i in &[0, 3] {
            let row = state.row_at(*i);
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
