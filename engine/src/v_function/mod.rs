use crate::game::Board;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

mod legacy;
mod n_tuple_medium;
mod n_tuple_small;

pub use self::legacy::*;
pub use self::n_tuple_medium::*;
pub use self::n_tuple_small::*;

pub trait Weights: Serialize + DeserializeOwned {
    fn optimized() -> Self;
}

pub trait VFunction: Debug {
    type Weights: Weights + Debug + Clone + Default;
    fn new(weights: Self::Weights) -> Self;
    fn eval(&self, state: Board) -> f32;
    fn learn(&mut self, state: Board, delta: f32);
    fn into_weights(self) -> Self::Weights;
}
