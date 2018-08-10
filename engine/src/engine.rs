use config::Config;
use game::{Board, Direction};
use lookup_table::LookupTable;
use std::iter::Iterator;

pub struct Engine {
    config: Config,
    eval_table: LookupTable<f32>,
}

impl Engine {
    pub fn new(config: Config) -> Engine {
        let eval_table = LookupTable::new(|row| {
            let mut eval = row.score() / 2.;
            eval += config.score_per_empty / 2. * (row.count_empties() as f32);
            eval += config.continuation_bonus / 8.;

            eval
        });

        Engine { config, eval_table }
    }

    pub fn search(&mut self, board: Board, depth: u64) -> Direction {
        assert!(depth > 0, "Depth must be greater than 0.");

        let moves = board.gen_moves();

        moves
            .into_iter()
            .map(|(dir, board)| (dir, self.expectimax_spawn_tile(board, depth)))
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap()
            .0
    }

    /// Searches the given board ang finds the best move
    fn expectimax_moves(&mut self, board: Board, depth: u64) -> f32 {
        if depth == 0 {
            return self.static_eval(board);
        }

        let moves = board.gen_moves();
        if moves.len() == 0 {
            return board.score();
        }

        moves
            .iter()
            .map(|(_, board)| self.expectimax_spawn_tile(*board, depth))
            .fold(0., |acc, value| if value > acc { value } else { acc })
    }

    fn expectimax_spawn_tile(&mut self, board: Board, depth: u64) -> f32 {
        let moves = board.gen_tile_spawns();

        moves
            .into_iter()
            .map(|(prob, board)| (prob, self.expectimax_moves(board, depth - 1)))
            .fold(0., |acc, (prob, score)| acc + prob * score)
    }

    /// Statically evaluates the given position by evaluating the expected score
    fn static_eval(&mut self, board: Board) -> f32 {
        let mut eval = 0.;

        for i in 0..4 {
            eval += self.eval_table[board.row_at(i)];
        }

        for i in 0..4 {
            eval += self.eval_table[board.column_at(i)];
        }

        eval
    }
}
