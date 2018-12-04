use game::{Board, Direction, TileSpawn};
use std::iter::Iterator;
use transposition_table::{PositionEval, TranspositionTable};
use v_function::VFunction;

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

    pub fn search(&mut self, board: Board, depth: u8) -> Direction {
        let moves = board.gen_moves();

        moves
            .into_iter()
            .map(|(dir, board)| (dir, self.expectimax_spawn_tile(board, depth)))
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap()
            .0
    }

    /// Searches the given board and finds the best move
    fn expectimax_moves(&mut self, board: Board, depth: u8) -> f32 {
        if depth >= 2 {
            match self.transposition_table.get(board) {
                Some(eval) if eval.depth >= depth => {
                    return eval.score;
                }
                _ => (),
            }
        }

        if depth == 0 {
            let score = self.static_eval(board);
            self.transposition_table
                .set(board, PositionEval::new(depth, score));
            return score;
        }

        let moves = board.gen_moves();
        if moves.len() == 0 {
            return board.score();
        }

        let score = moves
            .iter()
            .map(|(_, board)| self.expectimax_spawn_tile(*board, depth))
            .fold(0., |acc, value| if value > acc { value } else { acc });
        self.transposition_table
            .set(board, PositionEval::new(depth, score));
        score
    }

    fn expectimax_spawn_tile(&mut self, board: Board, depth: u8) -> f32 {
        let moves = board.gen_tile_spawns();

        moves
            .into_iter()
            .map(|(prob, tile, board)| {
                let new_depth = match tile {
                    TileSpawn::Two => depth - 1,
                    TileSpawn::Four => depth.saturating_sub(DEPTH_PENALTY_4),
                };

                (prob, self.expectimax_moves(board, new_depth))
            }).fold(0., |acc, (prob, score)| acc + prob * score)
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
