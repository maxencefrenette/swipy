use game::Board;

mod legacy;

pub use self::legacy::*;

pub trait VFunction {
    type Weights: Clone;
    fn new(weights: Self::Weights) -> Self;
    fn eval(&self, state: &Board) -> f32;
    fn learn(&mut self, state: &Board, delta: &f32);
    fn into_weights(self) -> Self::Weights;
}
