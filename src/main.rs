#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate tfe;

use clap::{App, AppSettings, Arg, SubCommand};

mod board;
mod config;
mod search;
mod stats;

use board::Board;
use config::{Config, OPTIMIZED_CONFIG};
use search::Search;
use stats::{mean, standard_dev};

fn main() {
    let app = init_clap();
    let matches = app.get_matches();

    match matches.subcommand_name().unwrap() {
        "play" => {
            let score = play_random_game(OPTIMIZED_CONFIG, true);
            println!("Final Score: {}", score);
        }
        "bench" => {
            let subcommand_matches = matches.subcommand_matches("bench").unwrap();
            let num_games = subcommand_matches
                .value_of("N")
                .expect("required arg")
                .parse::<u64>()
                .expect("number");

            let mut scores = Vec::with_capacity(num_games as usize);

            for i in 0..num_games {
                if i % 5 == 0 && i != 0 {
                    println!("{}/{}", i, num_games);
                }

                scores.push(play_random_game(OPTIMIZED_CONFIG, false) as f32);
            }

            let avg = mean(&scores);
            let sd = standard_dev(&scores, avg);
            let lower_bound = avg - 2. * sd;
            let upper_bound = avg + 2. * sd;

            println!("{} games played.", num_games);
            println!("Average score: {} +- {}", avg, sd);
            println!("Confidence interval: [{}, {}]", lower_bound, upper_bound);
        }
        _ => unreachable!(),
    }
}

fn init_clap<'a, 'b>() -> App<'a, 'b> {
    let play = SubCommand::with_name("play")
        .about("plays one game, logging the board to the command line");
    let bench = SubCommand::with_name("bench")
        .about("plays N games to test the strength of the AI")
        .arg(
            Arg::with_name("N")
                .help("The amount of games to play")
                .required(true)
                .takes_value(true),
        );

    App::new("Swipy - 2048 AI")
        .author(crate_authors!(", "))
        .version(crate_version!())
        .about("A 2048 AI")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommands(vec![play, bench])
}

fn play_random_game(config: Config, verbose: bool) -> u64 {
    let mut board = Board::new();
    let mut search = Search::new(config);

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
