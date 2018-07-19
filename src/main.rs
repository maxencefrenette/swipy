#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate tfe;

use clap::{App, AppSettings, SubCommand};

mod board;
mod search;

use board::Board;
use search::Search;

fn main() {
    let app = init_clap();
    let matches = app.get_matches();

    match matches.subcommand_name().unwrap() {
        "play" => {
            let score = play_random_game(true);
            println!("Final Score: {}", score);
        }
        "bench" => {
            let mut score = 0f64;

            for i in 0..100 {
                if i % 5 == 0 && i != 0 {
                    println!("{}", i);
                }

                score += play_random_game(false) as f64;
            }

            println!("100 games played.");
            println!("Average score: {}", score / 100.0);
        }
        _ => unreachable!(),
    }
}

fn init_clap<'a, 'b>() -> App<'a, 'b> {
    let play = SubCommand::with_name("play")
        .about("plays one game, logging the board to the command line");
    let bench =
        SubCommand::with_name("bench").about("plays N games to test the strength of the AI");

    App::new("My Super Program")
        .author(crate_authors!(", "))
        .version(crate_version!())
        .about("A 2048 AI")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommands(vec![play, bench])
}

fn play_random_game(verbose: bool) -> u64 {
    let mut board = Board::new();
    let mut search = Search::new();

    if verbose {
        println!("{:?}", board);
        println!();
    }

    while !board.is_dead() {
        let mov = search.search(board.clone(), 2);
        board = board.make_move(&mov);

        if verbose {
            println!("{:?}", board);
            println!();
        }
    }

    board.score()
}
