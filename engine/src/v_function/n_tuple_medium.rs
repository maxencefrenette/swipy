use super::{VFunction, Weights};
use crate::game::Board;
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};

lazy_static! {
    static ref OPTIMIZED: NTupleMediumWeights =
        serde_json::from_slice(include_bytes!("../../../networks/n_tuple_small.json"))
            .expect("valid legacy weights");
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NTupleMediumWeights {
    pub outer: Vec<f32>,
    pub inner: Vec<f32>,
}

impl Weights for NTupleMediumWeights {
    fn optimized() -> Self {
        OPTIMIZED.clone()
    }
}

#[derive(Debug, Clone, Default)]
pub struct NTupleMedium {
    weights: NTupleMediumWeights,
}

impl VFunction for NTupleMedium {
    type Weights = NTupleMediumWeights;

    fn new(weights: NTupleMediumWeights) -> Self {
        NTupleMedium { weights }
    }

    fn eval(&self, state: &Board) -> f32 {
        let mut eval = 0.;

        for i in &[0, 3] {
            let row = state.row_at(*i);
            let column = state.column_at(*i);

            for tuple in &[row, column] {
                eval += self.weights.outer[tuple.as_u16() as usize];
                eval += self.weights.outer[tuple.reversed().as_u16() as usize];
            }
        }

        for i in &[1, 2] {
            let row = state.row_at(*i);
            let column = state.column_at(*i);

            for tuple in &[row, column] {
                eval += self.weights.inner[tuple.as_u16() as usize];
                eval += self.weights.inner[tuple.reversed().as_u16() as usize];
            }
        }

        eval
    }

    fn learn(&mut self, state: &Board, delta: &f32) {
        let adjusted_delta = delta / 16.;

        for i in &[0, 3] {
            let row = state.row_at(*i);
            let column = state.column_at(*i);

            for tuple in &[row, column] {
                self.weights.outer[tuple.as_u16() as usize] += adjusted_delta;
                self.weights.outer[tuple.reversed().as_u16() as usize] += adjusted_delta;
            }
        }

        for i in &[1, 2] {
            let row = state.row_at(*i);
            let column = state.column_at(*i);

            for tuple in &[row, column] {
                self.weights.inner[tuple.as_u16() as usize] += adjusted_delta;
                self.weights.inner[tuple.reversed().as_u16() as usize] += adjusted_delta;
            }
        }
    }

    fn into_weights(self) -> NTupleMediumWeights {
        self.weights
    }
}
