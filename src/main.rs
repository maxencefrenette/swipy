#[macro_use]
extern crate lazy_static;
extern crate tfe;

mod board;

use board::Board;

fn main() {
    let mut b = Board::new();

    b = b.gen_moves()[2].1.clone();
    println!("{:?}", b);
    b = b.gen_tile_spawns()[2].1.clone();
    println!("{:?}", b);
    println!();

    b = b.gen_moves()[2].1.clone();
    println!("{:?}", b);
    b = b.gen_tile_spawns()[2].1.clone();
    println!("{:?}", b);
}
