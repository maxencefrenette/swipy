use super::row::Row;
use lookup_table::LookupTable;
use rand::{
    distributions::{Distribution, Weighted, WeightedChoice},
    thread_rng,
};
use std::fmt;
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

lazy_static! {
    static ref DIRECTIONS: Vec<Direction> = vec![
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ];
    static ref SCORE: LookupTable<f32> = LookupTable::new(|row| row.score());
    static ref MOVES: (
        LookupTable<Row>,
        LookupTable<Row>,
        LookupTable<u64>,
        LookupTable<u64>,
    ) = {
        let mut left: LookupTable<Row> = Default::default();
        let mut right: LookupTable<Row> = Default::default();
        let mut up: LookupTable<u64> = Default::default();
        let mut down: LookupTable<u64> = Default::default();

        for row in Row::iter_all() {
            let moved = row.moved();
            let rev_row = row.reversed();
            let rev_moved = moved.reversed();

            left[row] = moved;
            right[rev_row] = rev_moved;
            up[row] = moved.as_column();
            down[rev_row] = rev_moved.as_column();
        }

        (left, right, up, down)
    };
    static ref LEFT_MOVES: &'static LookupTable<Row> = &MOVES.0;
    static ref RIGHT_MOVES: &'static LookupTable<Row> = &MOVES.1;
    static ref UP_MOVES: &'static LookupTable<u64> = &MOVES.2;
    static ref DOWN_MOVES: &'static LookupTable<u64> = &MOVES.3;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Board(u64);

impl Board {
    pub fn new() -> Board {
        Board(0).spawn_random_tile().spawn_random_tile()
    }

    pub fn from_u64(bitboard: u64) -> Board {
        Board(bitboard)
    }

    pub fn at(&self, x: u64, y: u64) -> u64 {
        let tile_index = y * 4 + x;
        (self.0 >> (tile_index * 4)) & 0xF
    }

    pub fn row_at(&self, i: usize) -> Row {
        Row::from_u16((self.0 >> (i * 16)) as u16)
    }

    pub fn column_at(&self, i: usize) -> Row {
        let bb = self.0 >> 4 * i;

        let c1 = (bb & 0xF) as u16;
        let c2 = ((bb >> 12) & 0xF0) as u16;
        let c3 = ((bb >> 24) & 0xF00) as u16;
        let c4 = ((bb >> 36) & 0xF000) as u16;

        Row::from_u16(c4 | c3 | c2 | c1)
    }

    pub fn is_dead(&self) -> bool {
        self.gen_moves().len() == 0
    }

    pub fn score(&self) -> f32 {
        (0..4).map(|i| SCORE[self.row_at(i)]).sum()
    }

    pub fn count_empties(&self) -> u64 {
        let mut bitboard = self.0;
        let mut empties = 0;

        for _ in 0..16 {
            if bitboard & 0xF == 0 {
                empties += 1;
            }

            bitboard >>= 4;
        }

        empties
    }

    pub fn make_move(&self, direction: &Direction) -> Board {
        self.move_candidate(direction).spawn_random_tile()
    }

    pub fn gen_moves(&self) -> Vec<(Direction, Board)> {
        let mut result = Vec::<(Direction, Board)>::with_capacity(4);

        for dir in DIRECTIONS.iter() {
            let game2 = self.move_candidate(dir);
            if self.0 != game2.0 {
                result.push((*dir, Board(game2.0)))
            }
        }

        result
    }

    fn move_candidate(&self, direction: &Direction) -> Board {
        match direction {
            Direction::Left => {
                let mut res = 0;
                res |= (LEFT_MOVES[self.row_at(0)].as_u16() as u64) << 0;
                res |= (LEFT_MOVES[self.row_at(1)].as_u16() as u64) << 16;
                res |= (LEFT_MOVES[self.row_at(2)].as_u16() as u64) << 32;
                res |= (LEFT_MOVES[self.row_at(3)].as_u16() as u64) << 48;
                Board::from_u64(res)
            }
            Direction::Right => {
                let mut res = 0;
                res |= (RIGHT_MOVES[self.row_at(0)].as_u16() as u64) << 0;
                res |= (RIGHT_MOVES[self.row_at(1)].as_u16() as u64) << 16;
                res |= (RIGHT_MOVES[self.row_at(2)].as_u16() as u64) << 32;
                res |= (RIGHT_MOVES[self.row_at(3)].as_u16() as u64) << 48;
                Board::from_u64(res)
            }
            Direction::Up => {
                let transposed = self.transpose();
                let mut res = 0;
                res |= UP_MOVES[transposed.row_at(0)] << 0;
                res |= UP_MOVES[transposed.row_at(1)] << 4;
                res |= UP_MOVES[transposed.row_at(2)] << 8;
                res |= UP_MOVES[transposed.row_at(3)] << 12;
                Board::from_u64(res)
            }
            Direction::Down => {
                let transposed = self.transpose();
                let mut res = 0;
                res |= DOWN_MOVES[transposed.row_at(0)] << 0;
                res |= DOWN_MOVES[transposed.row_at(1)] << 4;
                res |= DOWN_MOVES[transposed.row_at(2)] << 8;
                res |= DOWN_MOVES[transposed.row_at(3)] << 12;
                Board::from_u64(res)
            }
        }
    }

    fn spawn_random_tile(&self) -> Board {
        let mut rng = thread_rng();
        let mut items: Vec<Weighted<Board>> = self
            .gen_tile_spawns()
            .into_iter()
            .map(|(prob, board)| Weighted {
                weight: (1000000. * prob) as u32,
                item: board,
            })
            .collect();
        let wc = WeightedChoice::new(&mut items);

        wc.sample(&mut rng)
    }

    pub fn gen_tile_spawns(&self) -> Vec<(f32, Board)> {
        let mut results = Vec::<(f32, Board)>::new();

        for i in 0..16 {
            if (self.0 >> (i * 4)) & 0xF == 0 {
                results.push((0.9, Board(self.0 | 1 << (i * 4))));
                results.push((0.1, Board(self.0 | 2 << (i * 4))));
            }
        }

        results
    }

    fn transpose(&self) -> Board {
        let x = self.0;
        let a1 = x & 0xF0F00F0FF0F00F0F;
        let a2 = x & 0x0000F0F00000F0F0;
        let a3 = x & 0x0F0F00000F0F0000;
        let a = a1 | (a2 << 12) | (a3 >> 12);
        let b1 = a & 0xFF00FF0000FF00FF;
        let b2 = a & 0x00FF00FF00000000;
        let b3 = a & 0x00000000FF00FF00;
        Board::from_u64(b1 | (b2 >> 24) | (b3 << 24))
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..4 {
            writeln!(f, "{:?}", self.row_at(i))?;
        }

        Ok(())
    }
}
