#[macro_use]
extern crate lazy_static;
extern crate tfe;

mod board;
mod search;

use board::Board;
use search::Search;

fn main() {
    let score = play_random_game();
    println!("Final Score: {}", score);
}

fn play_random_game() -> u64 {
    let mut board = Board::new();
    let mut search = Search::new();

    println!("{:?}", board);
    println!();

    while !board.is_dead() {
        let mov = search.search(board.clone(), 2);
        board = board.make_move(&mov);

        println!("{:?}", board);
        println!();
    }

    board.score()
}
