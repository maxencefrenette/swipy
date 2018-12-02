use config::Config;
use game::{Board, Direction, TileSpawn};
use std::iter::Iterator;
use transposition_table::{PositionEval, TranspositionTable};

/// The search depth counter increase when processing a move where a 4 spawns.
/// This is approximately equal to ln(0.1) / ln(0.9) = 21.85434532678.
const DEPTH_PENALTY_4: u8 = 22;

pub struct Engine {
    config: Config,
    transposition_table: TranspositionTable,
}

impl Engine {
    pub fn new(config: Config) -> Engine {
        let transposition_table = TranspositionTable::new(0x1000);

        Engine {
            config,
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
        let mut eval = 0.;

        for i in 0..4 {
            let row = position.row_at(i);
            let column = position.column_at(i);

            eval += row.score() / 2.;
            eval += column.score() / 2.;

            eval += self.config.outer_pos_bonus[row.tile_at(0) as usize];
            eval += self.config.inner_pos_bonus[row.tile_at(1) as usize];
            eval += self.config.inner_pos_bonus[row.tile_at(2) as usize];
            eval += self.config.outer_pos_bonus[row.tile_at(3) as usize];
            eval += self.config.outer_pos_bonus[column.tile_at(0) as usize];
            eval += self.config.inner_pos_bonus[column.tile_at(1) as usize];
            eval += self.config.inner_pos_bonus[column.tile_at(2) as usize];
            eval += self.config.outer_pos_bonus[column.tile_at(3) as usize];
        }

        eval
    }

    /// Resets the state of the engine as if it was new
    pub fn reset(&mut self) {
        self.transposition_table.clear();
    }

    pub fn learn(&mut self, position: &Board, delta: &f32) {
        let adjusted_delta = delta / 8.;

        for i in 0..4 {
            let row = position.row_at(i);
            self.config.outer_pos_bonus[row.tile_at(0) as usize] += adjusted_delta;
            self.config.inner_pos_bonus[row.tile_at(1) as usize] += adjusted_delta;
            self.config.inner_pos_bonus[row.tile_at(2) as usize] += adjusted_delta;
            self.config.outer_pos_bonus[row.tile_at(3) as usize] += adjusted_delta;

            let column = position.column_at(i);
            self.config.outer_pos_bonus[column.tile_at(0) as usize] += adjusted_delta;
            self.config.inner_pos_bonus[column.tile_at(1) as usize] += adjusted_delta;
            self.config.inner_pos_bonus[column.tile_at(2) as usize] += adjusted_delta;
            self.config.outer_pos_bonus[column.tile_at(3) as usize] += adjusted_delta;
        }
    }

    /// Destroys the engine and returns the config
    pub fn into_config(self) -> Config {
        self.config
    }
}
