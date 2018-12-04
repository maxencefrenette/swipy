#[macro_use]
extern crate clap;
extern crate indicatif;
extern crate statistical;
extern crate swipy_engine;

use clap::{App, AppSettings, Arg, SubCommand};
use indicatif::{ProgressBar, ProgressStyle};
use statistical::{mean, standard_deviation, univariate::standard_error_mean};
use std::str::FromStr;
use swipy_engine::{train_td, Board, Engine, DEFAULT_CONFIG, OPTIMIZED_CONFIG};

const DEPTH: u8 = 2;

fn main() {
    let app = init_clap();
    let matches = app.get_matches();

    match matches.subcommand_name().unwrap() {
        "play" => {
            let mut engine = Engine::new(OPTIMIZED_CONFIG);
            let board = play_random_game(&mut engine, DEPTH, true);
            println!("Final Score: {}", board.score());
        }
        "bench" => {
            // Parse arguments
            let subcommand_matches = matches.subcommand_matches("bench").unwrap();
            let num_games = subcommand_matches
                .value_of("N")
                .expect("required arg")
                .parse::<u64>()
                .expect("number");

            let init_engine_bar = ProgressBar::new(1);
            init_engine_bar.set_message("[1/2] Initializing precomputed tables");
            init_engine_bar.set_style(ProgressStyle::default_spinner().template("{msg} {spinner}"));

            // Init engine
            let mut scores = Vec::with_capacity(num_games as usize);
            let mut tiles_reached = [0u64; 16];
            let mut engine = Engine::new(OPTIMIZED_CONFIG);
            init_engine_bar.finish();

            let play_games_bar = ProgressBar::new(num_games);
            play_games_bar.set_message("[2/2] Playing games");
            play_games_bar
                .set_style(ProgressStyle::default_bar().template("{msg} {wide_bar} {eta}"));
            play_games_bar.tick();

            for _ in 0..num_games {
                let board = play_random_game(&mut engine, DEPTH, false);

                scores.push(board.score());

                for i in 0..board.highest_tile() {
                    tiles_reached[i as usize] += 1;
                }

                play_games_bar.inc(1);
                engine.reset();
            }

            let avg = mean(scores.as_slice());
            let sd = standard_deviation(scores.as_slice(), Some(avg));
            let err = standard_error_mean(sd, scores.len() as f32, None);
            let lower_bound = avg - 1.96 * err;
            let upper_bound = avg + 1.96 * err;

            play_games_bar.finish();
            println!();

            println!("{} games played.", num_games);
            println!("Average score: {:.0} \u{00b1} {:.0}", avg, err);
            println!("Standard deviation: {:.0}", sd);
            println!(
                "Confidence interval (95%): [{:.0}, {:.0}]",
                lower_bound, upper_bound
            );
            println!();

            for n in 8..13 {
                println!(
                    "{}: {}%",
                    u64::pow(2, n),
                    (tiles_reached[n as usize] as f32) * 100. / (num_games as f32)
                );
            }
        }
        "train" => {
            let subcommand_matches = matches.subcommand_matches("train").unwrap();
            let num_batches = subcommand_matches
                .value_of("N")
                .expect("required arg")
                .parse::<u64>()
                .expect("number");

            let zero = subcommand_matches.is_present("zero");
            let alpha = f32::from_str(subcommand_matches.value_of("alpha").unwrap()).unwrap();

            train(num_batches, alpha, zero);
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
    let train = SubCommand::with_name("train")
        .about("continuously plays to optimize the AI")
        .arg(
            Arg::with_name("zero")
                .short("z")
                .help("Starts training from scratch"),
        ).arg(
            // TODO: Add a validator here
            Arg::with_name("alpha")
                .long("alpha")
                .takes_value(true)
                .default_value("0.0005")
                .help("The learning rate"),
        ).arg(
            Arg::with_name("N")
                .help("The amount of batches of 5 games to play")
                .required(true)
                .takes_value(true),
        );

    App::new("Swipy - 2048 AI")
        .author(crate_authors!(", "))
        .version(crate_version!())
        .about("A 2048 AI")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommands(vec![play, bench, train])
}

fn play_random_game(engine: &mut Engine, depth: u8, verbose: bool) -> Board {
    let mut board = Board::new();

    if verbose {
        println!("{:?}", board);
        println!();
    }

    while !board.is_dead() {
        let mov = engine.search(board, depth);
        board = board.make_move(&mov);

        if verbose {
            println!("{:?}", board);
            println!();
        }
    }

    board
}

fn train(num_batches: u64, alpha: f32, zero: bool) {
    let config = match zero {
        true => DEFAULT_CONFIG,
        false => OPTIMIZED_CONFIG,
    };

    let new_config = train_td(&mut config.clone(), &num_batches, &alpha);

    println!("{:?}", new_config);
}
