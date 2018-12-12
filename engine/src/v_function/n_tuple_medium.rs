use super::{VFunction, Weights};
use crate::game::Board;
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};

lazy_static! {
    static ref OPTIMIZED: NTupleMediumWeights =
        serde_json::from_slice(include_bytes!("../../../networks/n_tuple_small.json"))
            .expect("valid legacy weights");
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NTupleMediumWeights {
    pub outer: Vec<f32>,
    pub inner: Vec<f32>,
}

impl Default for NTupleMediumWeights {
    fn default() -> Self {
        NTupleMediumWeights {
            outer: vec![0.; 0xF_0000],
            inner: vec![0.; 0xF_0000],
        }
    }
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

    fn eval(&self, state: Board) -> f32 {
        let mut eval = 0.;

        for i in &[0, 3] {
            let row = state.row_at(*i);
            let column = state.column_at(*i);

            for tuple in &[row, column] {
                eval += self.weights.outer[tuple.into_usize()];
                eval += self.weights.outer[tuple.reversed().into_usize()];
            }
        }

        for i in &[1, 2] {
            let row = state.row_at(*i);
            let column = state.column_at(*i);

            for tuple in &[row, column] {
                eval += self.weights.inner[tuple.into_usize()];
                eval += self.weights.inner[tuple.reversed().into_usize()];
            }
        }

        eval
    }

    fn learn(&mut self, state: Board, delta: f32) {
        let adjusted_delta = delta / 16.;

        for i in &[0, 3] {
            let row = state.row_at(*i);
            let column = state.column_at(*i);

            for tuple in &[row, column] {
                self.weights.outer[tuple.into_usize()] += adjusted_delta;
                self.weights.outer[tuple.reversed().into_usize()] += adjusted_delta;
            }
        }

        for i in &[1, 2] {
            let row = state.row_at(*i);
            let column = state.column_at(*i);

            for tuple in &[row, column] {
                self.weights.inner[tuple.into_usize()] += adjusted_delta;
                self.weights.inner[tuple.reversed().into_usize()] += adjusted_delta;
            }
        }
    }

    fn into_weights(self) -> NTupleMediumWeights {
        self.weights
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    const BOARD_1: Board =
        Board::from_array([[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]]);
    const BOARD_2: Board =
        Board::from_array([[0, 1, 2, 3], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);

    #[test]
    fn eval_zero() {
        let default = NTupleMedium::default();
        assert_relative_eq!(default.eval(BOARD_1), 0.);
    }

    #[test]
    fn training_eval() {
        let mut network = NTupleMedium::default();
        network.learn(BOARD_1, 1.0);
        assert_relative_eq!(network.eval(BOARD_1), 1.0);
    }

    #[test]
    fn generalization() {
        let mut network = NTupleMedium::default();
        network.learn(BOARD_1, 1.0);
        assert_relative_eq!(network.eval(BOARD_2), 1. / 8.);
    }
}
