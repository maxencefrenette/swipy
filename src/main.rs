#[macro_use]
extern crate clap;
extern crate statistical;
extern crate swipy_engine;

use clap::{App, AppSettings, Arg, SubCommand};
use statistical::{mean, standard_deviation, univariate::standard_error_mean};
use swipy_engine::{Board, Config, Engine, OPTIMIZED_CONFIG};

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

            let avg = mean(scores.as_slice());
            let sd = standard_deviation(scores.as_slice(), Some(avg));
            let err = standard_error_mean(sd, scores.len() as f32, None);
            let lower_bound = avg - 1.96 * err;
            let upper_bound = avg + 1.96 * err;

            println!("{} games played.", num_games);
            println!("Average score: {:.0} \u{00b1} {:.0}", avg, err);
            println!("Standard deviation: {:.0}", sd);
            println!(
                "Confidence interval (95%): [{:.0}, {:.0}]",
                lower_bound, upper_bound
            );
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
    let mut engine = Engine::new(config);

    if verbose {
        println!("{:?}", board);
        println!();
    }

    while !board.is_dead() {
        let mov = engine.search(board.clone(), 2);
        board = board.make_move(&mov);

        if verbose {
            println!("{:?}", board);
            println!();
        }
    }

    board.score()
}
