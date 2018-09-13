use fnv::FnvHasher;
use game::Board;
use std::hash::{Hash, Hasher};
use std::iter::repeat;

/// The result of a single position evaluation at a certain depth
#[derive(Debug, Clone, Copy)]
pub struct PositionEval {
    pub depth: u8,
    pub score: f32,
}

impl PositionEval {
    pub fn new(depth: u8, score: f32) -> PositionEval {
        PositionEval { depth, score }
    }
}

pub struct TranspositionTable(Vec<Option<(Board, PositionEval)>>);

impl TranspositionTable {
    pub fn new(size: usize) -> TranspositionTable {
        TranspositionTable(repeat(None).take(size).collect())
    }

    pub fn get(&self, board: Board) -> Option<PositionEval> {
        if let Some((entry_board, eval)) = self.0[self.bucket_of(board)] {
            if entry_board == board {
                return Some(eval);
            }
        }

        None
    }

    pub fn set(&mut self, board: Board, eval: PositionEval) {
        let i = self.bucket_of(board);

        if let Some((entry_board, entry_eval)) = self.0[i] {
            if entry_board == board && entry_eval.depth <= eval.depth {
                self.0[i] = Some((board, eval));
            }
        } else {
            self.0[i] = Some((board, eval));
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.0.len() {
            self.0[i] = None;
        }
    }

    fn hash(board: Board) -> u64 {
        let mut hasher = FnvHasher::default();
        board.hash(&mut hasher);
        hasher.finish()
    }

    fn bucket_of(&self, board: Board) -> usize {
        (Self::hash(board) as usize) % self.0.len()
    }
}
