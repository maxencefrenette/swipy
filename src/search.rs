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
            .max_by_key(|t| self.expectimax_spawn_tile(t.1.clone(), depth))
            .unwrap()
            .0
    }

    /// Searches the given board ang finds the best move
    fn expectimax_moves(&mut self, board: Board, depth: u64) -> u64 {
        if depth == 0 {
            return self.static_eval(board);
        }

        let moves = board.gen_moves();
        if moves.len() == 0 {
            return board.score();
        }

        moves
            .into_iter()
            .map(|(_, board)| self.expectimax_spawn_tile(board, depth))
            .max()
            .unwrap()
    }

    fn expectimax_spawn_tile(&mut self, board: Board, depth: u64) -> u64 {
        let moves = board.gen_tile_spawns();

        let score = moves
            .into_iter()
            .map(|(prob, board)| (prob, self.expectimax_moves(board, depth - 1)))
            .fold(0f64, |acc, (prob, score)| acc + prob * (score as f64));

        score as u64
    }

    /// Statically evaluates the given position
    fn static_eval(&mut self, board: Board) -> u64 {
        board.score()
    }
}
