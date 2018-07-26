use row::Row;
use std::ops::Index;
use std::ops::IndexMut;

const TABLE_SIZE: usize = 0x1_0000;

pub struct LookupTable<T>(Vec<T>);

impl<T> LookupTable<T> {
    pub fn new<F>(calc: F) -> LookupTable<T>
    where
        F: Fn(Row) -> T,
    {
        LookupTable(
            (0..TABLE_SIZE)
                .map(|row| calc(Row::from_u16(row as u16)))
                .collect(),
        )
    }
}

impl<T> Index<Row> for LookupTable<T> {
    type Output = T;

    fn index(&self, row: Row) -> &T {
        &self.0[row.as_u16() as usize]
    }
}

impl<T> IndexMut<Row> for LookupTable<T> {
    fn index_mut(&mut self, row: Row) -> &mut T {
        &mut self.0[row.as_u16() as usize]
    }
}

impl<T> Default for LookupTable<T>
where
    T: Default + Clone,
{
    fn default() -> LookupTable<T> {
        LookupTable(vec![Default::default(); TABLE_SIZE])
    }
}
