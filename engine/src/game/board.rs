use super::row::Row;
use crate::lookup_table::LookupTable;
use lazy_static::lazy_static;
use rand::{
    distributions::{Distribution, WeightedIndex},
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileSpawn {
    Two,
    Four,
}

impl TileSpawn {
    pub fn prob(self) -> f32 {
        match self {
            TileSpawn::Two => 0.9,
            TileSpawn::Four => 0.1,
        }
    }

    pub fn relative_prob(self) -> u32 {
        match self {
            TileSpawn::Two => 9,
            TileSpawn::Four => 1,
        }
    }
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board(u64);

impl Board {
    pub fn new_random() -> Board {
        Board(0).spawn_random_tile().spawn_random_tile()
    }

    pub fn from_u64(bitboard: u64) -> Board {
        Board(bitboard)
    }

    pub fn at(self, x: u64, y: u64) -> u64 {
        let tile_index = y * 4 + x;
        (self.0 >> (tile_index * 4)) & 0xF
    }

    pub fn row_at(self, i: usize) -> Row {
        Row::from_u16((self.0 >> (i * 16)) as u16)
    }

    pub fn column_at(self, i: usize) -> Row {
        let bb = self.0 >> (4 * i);

        let c1 = (bb & 0xF) as u16;
        let c2 = ((bb >> 12) & 0xF0) as u16;
        let c3 = ((bb >> 24) & 0xF00) as u16;
        let c4 = ((bb >> 36) & 0xF000) as u16;

        Row::from_u16(c4 | c3 | c2 | c1)
    }

    pub fn is_dead(self) -> bool {
        self.gen_moves().is_empty()
    }

    pub fn score(self) -> f32 {
        (0..4).map(|i| SCORE[self.row_at(i)]).sum()
    }

    pub fn highest_tile(self) -> u64 {
        (0..16).map(|i| self.at(i / 4, i % 4)).max().unwrap()
    }

    #[allow(clippy::verbose_bit_mask)]
    pub fn count_empties(self) -> u64 {
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

    pub fn make_move(self, direction: Direction) -> Board {
        self.move_candidate(direction).spawn_random_tile()
    }

    pub fn gen_moves(self) -> Vec<(Direction, Board)> {
        let mut result = Vec::<(Direction, Board)>::with_capacity(4);

        for dir in DIRECTIONS.iter() {
            let game2 = self.move_candidate(*dir);
            if self.0 != game2.0 {
                result.push((*dir, Board(game2.0)))
            }
        }

        result
    }

    pub fn move_candidate(self, direction: Direction) -> Board {
        match direction {
            Direction::Left => {
                let mut res = 0;
                res |= u64::from(LEFT_MOVES[self.row_at(0)].as_u16());
                res |= u64::from(LEFT_MOVES[self.row_at(1)].as_u16()) << 16;
                res |= u64::from(LEFT_MOVES[self.row_at(2)].as_u16()) << 32;
                res |= u64::from(LEFT_MOVES[self.row_at(3)].as_u16()) << 48;
                Board::from_u64(res)
            }
            Direction::Right => {
                let mut res = 0;
                res |= u64::from(RIGHT_MOVES[self.row_at(0)].as_u16());
                res |= u64::from(RIGHT_MOVES[self.row_at(1)].as_u16()) << 16;
                res |= u64::from(RIGHT_MOVES[self.row_at(2)].as_u16()) << 32;
                res |= u64::from(RIGHT_MOVES[self.row_at(3)].as_u16()) << 48;
                Board::from_u64(res)
            }
            Direction::Up => {
                let transposed = self.transpose();
                let mut res = 0;
                res |= UP_MOVES[transposed.row_at(0)];
                res |= UP_MOVES[transposed.row_at(1)] << 4;
                res |= UP_MOVES[transposed.row_at(2)] << 8;
                res |= UP_MOVES[transposed.row_at(3)] << 12;
                Board::from_u64(res)
            }
            Direction::Down => {
                let transposed = self.transpose();
                let mut res = 0;
                res |= DOWN_MOVES[transposed.row_at(0)];
                res |= DOWN_MOVES[transposed.row_at(1)] << 4;
                res |= DOWN_MOVES[transposed.row_at(2)] << 8;
                res |= DOWN_MOVES[transposed.row_at(3)] << 12;
                Board::from_u64(res)
            }
        }
    }

    fn spawn_random_tile(self) -> Board {
        let mut rng = thread_rng();
        let tile_spawns: Vec<(u32, Board)> = self
            .gen_tile_spawns()
            .into_iter()
            .map(|(_prob, tile, board)| (tile.relative_prob(), board))
            .collect();

        let probabilities: Vec<u32> = tile_spawns.iter().map(|tile_spawn| tile_spawn.0).collect();
        let resulting_boards: Vec<Board> =
            tile_spawns.iter().map(|tile_spawn| tile_spawn.1).collect();

        resulting_boards[WeightedIndex::new(probabilities).unwrap().sample(&mut rng)]
    }

    #[allow(clippy::verbose_bit_mask)]
    pub fn gen_tile_spawns(self) -> Vec<(f32, TileSpawn, Board)> {
        let mut results = Vec::<(f32, TileSpawn, Board)>::new();
        let n = self.count_empties() as f32;

        for i in 0..16 {
            if (self.0 >> (i * 4)) & 0xF == 0 {
                results.push((
                    TileSpawn::Two.prob() / n,
                    TileSpawn::Two,
                    Board(self.0 | 1 << (i * 4)),
                ));
                results.push((
                    TileSpawn::Four.prob() / n,
                    TileSpawn::Four,
                    Board(self.0 | 2 << (i * 4)),
                ));
            }
        }

        results
    }

    fn transpose(self) -> Board {
        let x = self.0;
        let a1 = x & 0xF0F0_0F0F_F0F0_0F0F;
        let a2 = x & 0x0000_F0F0_0000_F0F0;
        let a3 = x & 0x0F0F_0000_0F0F_0000;
        let a = a1 | (a2 << 12) | (a3 >> 12);
        let b1 = a & 0xFF00_FF00_00FF_00FF;
        let b2 = a & 0x00FF_00FF_0000_0000;
        let b3 = a & 0x0000_0000_FF00_FF00;
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
