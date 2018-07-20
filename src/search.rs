use board::Board;
use std::iter::Iterator;
use tfe::Direction;

pub struct Search {}

impl Search {
    pub fn new() -> Search {
        Search {}
    }

    pub fn search(&mut self, board: Board, depth: u64) -> Direction {
        assert!(depth > 0, "Depth must be greater than 0.");

        let moves = board.gen_moves();

        moves
            .into_iter()
            .map(|(dir, board)| (dir, self.expectimax_spawn_tile(board.clone(), depth)))
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
            return board.score() as f32;
        }

        moves
            .iter()
            .map(|(_, board)| self.expectimax_spawn_tile(board.clone(), depth))
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
        let mut eval = board.score() as f32;
        eval += 100. * (board.count_empties() as f32);
        eval += 100.;

        eval
    }
}
