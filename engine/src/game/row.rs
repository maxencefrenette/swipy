use std::fmt;

const TILE_MASK: u16 = 0xF;
const COL_MASK: u64 = 0x000F_000F_000F_000F;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Row(u16);

impl Row {
    pub fn iter_all() -> impl Iterator<Item = Row> {
        (0..=0xFFFF).map(|i| Row::from_u16(i))
    }

    pub fn new(tiles: &[u16]) -> Row {
        assert!(tiles.len() == 4);
        Row((tiles[0] << 0) | (tiles[1] << 4) | (tiles[2] << 8) | (tiles[3] << 12))
    }

    pub fn from_u16(bits: u16) -> Row {
        Row(bits)
    }

    pub fn as_u16(&self) -> u16 {
        self.0 as u16
    }

    pub fn tile_at(&self, i: usize) -> u16 {
        assert!(i < 4, "i is in the interval 0..4");
        (self.0 >> (i * 4)) & TILE_MASK
    }

    pub fn score(&self) -> f32 {
        (0..4)
            .map(|i| {
                let tile = self.tile_at(i) as u32;
                if tile > 1 {
                    ((tile - 1) * (1 << tile)) as f32
                } else {
                    0.
                }
            }).sum()
    }

    pub fn count_empties(&self) -> u64 {
        let mut bitboard = self.0;
        let mut empties = 0;

        for _ in 0..4 {
            if bitboard & 0xF == 0 {
                empties += 1;
            }

            bitboard >>= 4;
        }

        empties
    }

    pub fn moved(&self) -> Row {
        let mut tiles: Vec<u16> = (0..4).map(|i| self.tile_at(i)).collect();

        let mut changed = true;
        while changed {
            changed = false;

            for i in 0..3 {
                if tiles[i] == 0 && tiles[i + 1] != 0 {
                    tiles[i] = tiles[i + 1];
                    tiles[i + 1] = 0;
                    changed = true;
                } else if tiles[i] != 0 && tiles[i] == tiles[i + 1] {
                    // Pretend that two 15 tiles can't be merged
                    if tiles[i] != 15 {
                        tiles[i] += 1;
                        tiles[i + 1] = 0;
                        changed = true;
                    }
                }
            }
        }

        Row::new(tiles.as_slice())
    }

    pub fn reversed(&self) -> Row {
        Row::new(&[
            self.tile_at(3),
            self.tile_at(2),
            self.tile_at(1),
            self.tile_at(0),
        ])
    }

    pub fn as_column(&self) -> u64 {
        let tmp = self.0 as u64;
        (tmp | (tmp << 12) | (tmp << 24) | (tmp << 36)) & COL_MASK
    }
}

impl fmt::Debug for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..4 {
            let tile = self.tile_at(i);
            write!(f, "| {} ", repr_tile(tile))?;
        }
        writeln!(f, "|")?;
        Ok(())
    }
}

fn repr_tile(tile: u16) -> u64 {
    match tile {
        0 => 0,
        n if 1 <= n && n <= 15 => 1 << n,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(Row::new(&[1, 2, 3, 4]), Row(0x4321));
    }

    #[test]
    fn score() {
        assert_eq!(Row::new(&[0, 2, 0, 0]).score(), 4.);
        assert_eq!(Row::new(&[0, 4, 4, 0]).score(), 96.);
    }

    #[test]
    fn moved() {
        assert_eq!(Row::new(&[1, 1, 0, 0]).moved(), Row::new(&[2, 0, 0, 0]));
        assert_eq!(Row::new(&[0, 0, 1, 0]).moved(), Row::new(&[1, 0, 0, 0]));
        assert_eq!(Row::new(&[0, 1, 3, 3]).moved(), Row::new(&[1, 4, 0, 0]));
    }

    #[test]
    fn moved_overflow() {
        assert_eq!(Row::new(&[15, 15, 0, 0]).moved(), Row::new(&[15, 15, 0, 0]));
    }

    #[test]
    fn reversed() {
        assert_eq!(Row::new(&[1, 2, 3, 4]).reversed(), Row::new(&[4, 3, 2, 1]));
    }

    #[test]
    fn as_column() {
        assert_eq!(Row::new(&[1, 2, 3, 4]).as_column(), 0x0004_0003_0002_0001);
    }
}
