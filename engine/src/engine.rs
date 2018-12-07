use crate::game::{Board, Direction, TileSpawn};
use std::iter::Iterator;
use crate::transposition_table::{PositionEval, TranspositionTable};
use crate::v_function::VFunction;

/// The search depth counter increase when processing a move where a 4 spawns.
/// This is approximately equal to ln(0.1) / ln(0.9) = 21.85434532678.
const DEPTH_PENALTY_4: u8 = 22;

pub struct Engine<F>
where
    F: VFunction,
{
    v_function: F,
    transposition_table: TranspositionTable,
}

impl<F> Engine<F>
where
    F: VFunction,
{
    pub fn new(weights: F::Weights) -> Self {
        let transposition_table = TranspositionTable::new(0x1000);

        Engine {
            v_function: F::new(weights),
            transposition_table,
        }
    }

    /// Recursively searches for the best move to perform with the current game state
    /// Uses afterstates as leaves to statically evaluate
    pub fn search(&mut self, board: Board, depth: u8) -> Direction {
        let moves = board.gen_moves();

        moves
            .into_iter()
            .map(|(dir, board)| (dir, self.expectimax_spawn_tile(board, depth - 1)))
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).expect("values are not NAN"))
            .expect("moves has elements")
            .0
    }

    /// Evaluates the expected score of a position using expectimax.
    ///
    /// The `board` argument represents a state of the board between turns.
    fn expectimax_move(&mut self, board: Board, depth: u8) -> f32 {
        let moves = board.gen_moves();
        if moves.len() == 0 {
            return 0.;
        }

        moves
            .iter()
            .map(|(_, next_board)| {
                self.expectimax_spawn_tile(*next_board, depth)
                    + (next_board.score() - board.score())
            }).max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }

    /// Evaluates the expected score of an afterstate using expectimax.
    ///
    /// The `board` argument represents an afterstate of the board, which is the state a board
    /// takes after a move has been made, but before a random tile has appeared.
    fn expectimax_spawn_tile(&mut self, board: Board, depth: u8) -> f32 {
        if depth >= 2 {
            match self.transposition_table.get(board) {
                Some(eval) if eval.depth >= depth => {
                    return eval.score;
                }
                _ => (),
            }
        }

        if depth == 0 {
            return self.static_eval(board);
        }

        let moves = board.gen_tile_spawns();

        let score = moves
            .into_iter()
            .map(|(prob, tile, board)| {
                let new_depth = match tile {
                    TileSpawn::Two => depth - 1,
                    TileSpawn::Four => depth.saturating_sub(DEPTH_PENALTY_4),
                };

                prob * self.expectimax_move(board, new_depth)
            }).sum();

        self.transposition_table
            .set(board, PositionEval::new(depth, score));

        score
    }

    /// Statically evaluates the given position by evaluating the expected score
    pub fn static_eval(&self, position: Board) -> f32 {
        self.v_function.eval(&position)
    }

    pub fn learn(&mut self, position: &Board, delta: &f32) {
        self.v_function.learn(position, delta)
    }

    /// Destroys the engine and returns the config
    pub fn into_weights(self) -> F::Weights {
        self.v_function.into_weights()
    }

    /// Resets the state of the engine as if it was new
    pub fn reset(&mut self) {
        self.transposition_table.clear();
    }
}
