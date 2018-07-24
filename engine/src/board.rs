use std::fmt;
use std::vec::Vec;
use tfe::{Direction, Game};

lazy_static! {
    static ref DIRECTIONS: Vec<Direction> = vec![
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ];
}

pub struct Board {
    inner: Game,
}

impl Board {
    pub fn new() -> Board {
        Board { inner: Game::new() }
    }

    pub fn from_u64(bitboard: u64) -> Board {
        Board {
            inner: Game { board: bitboard },
        }
    }

    pub fn at(&self, x: u64, y: u64) -> u64 {
        let tile_index = y * 4 + x;
        (self.inner.board >> (tile_index * 4)) & 0xF
    }

    pub fn is_dead(&self) -> bool {
        self.gen_moves().len() == 0
    }

    pub fn score(&self) -> u64 {
        let mut s = 0;

        for i in 0..4 {
            for j in 0..4 {
                let tile = self.at(i, j);
                if tile > 1 {
                    s += (tile - 1) * (2 << tile);
                }
            }
        }

        s
    }

    pub fn count_empties(&self) -> u64 {
        let mut bitboard = self.inner.board;
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
        let shifted = Game::execute(self.inner.board, direction);
        let spawned_tile = Game::spawn_tile(shifted);

        Self::from_u64(shifted | spawned_tile)
    }

    pub fn gen_moves(&self) -> Vec<(Direction, Board)> {
        let mut result = Vec::<(Direction, Board)>::with_capacity(4);

        for dir in DIRECTIONS.iter() {
            let game2 = Game::execute(self.inner.board, dir);
            if self.inner.board != game2 {
                result.push((
                    dir.clone(),
                    Board {
                        inner: Game { board: game2 },
                    },
                ))
            }
        }

        result
    }

    pub fn gen_tile_spawns(&self) -> Vec<(f32, Board)> {
        let mut results = Vec::<(f32, Board)>::new();
        let mut tmp = self.inner.board;

        for i in 0..16 {
            tmp >>= 4;

            if (tmp & 0xF) == 0 {
                results.push((
                    0.9,
                    Board {
                        inner: Game {
                            board: self.inner.board | 1 << (i * 4),
                        },
                    },
                ));
                results.push((
                    0.1,
                    Board {
                        inner: Game {
                            board: self.inner.board | 2 << (i * 4),
                        },
                    },
                ));
            }
        }

        results
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..4 {
            for j in 0..4 {
                let tile = self.at(j, i);
                write!(f, "| {} ", repr_tile(tile))?;
            }
            writeln!(f, "|")?;
        }

        Ok(())
    }
}

impl Clone for Board {
    fn clone(&self) -> Board {
        Board {
            inner: Game {
                board: self.inner.board,
            },
        }
    }
}

fn repr_tile(tile: u64) -> u64 {
    match tile {
        0 => 0,
        n if 1 <= n && n <= 15 => 1 << n,
        _ => unreachable!(),
    }
}
