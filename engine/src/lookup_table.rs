use std::ops::Index;

const TABLE_SIZE: usize = 0x1_000_000;

pub struct LookupTable<T>(Vec<T>);

impl<T> LookupTable<T> {
    pub fn new<F>(calc: F) -> LookupTable<T>
    where
        F: Fn(u16) -> T,
    {
        LookupTable((0..TABLE_SIZE).map(|row| calc(row as u16)).collect())
    }
}

impl<T> Index<u16> for LookupTable<T> {
    type Output = T;

    fn index(&self, row: u16) -> &T {
        &self.0[row as usize]
    }
}
