#[macro_use]
extern crate clap;
extern crate cmaes;
extern crate indicatif;
extern crate statistical;
extern crate swipy_engine;

use clap::{App, AppSettings, Arg, SubCommand};
use cmaes::*;
use indicatif::{ProgressBar, ProgressStyle};
use statistical::{mean, standard_deviation, univariate::standard_error_mean};
use swipy_engine::{Board, Config, Engine, DEFAULT_CONFIG, OPTIMIZED_CONFIG};

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

            train(num_batches, zero);
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
                .help("starts training from scratch"),
        )
        .arg(
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

#[derive(Clone)]
struct FitnessEvaluator;

impl FitnessFunction for FitnessEvaluator {
    fn get_fitness(&self, parameters: &[f64]) -> f64 {
        let batch_size = 10;
        let config = Config::from_vec(parameters.iter().map(|p| (*p as f32)).collect());
        let mut engine = Engine::new(config);

        let mut score: f64 = 0.;

        for _ in 0..batch_size {
            let board = play_random_game(&mut engine, DEPTH, false);
            score += board.score() as f64;
        }
        score /= batch_size as f64;

        println!("Score: {:05.0}", score);

        -score
    }
}

fn train(num_batches: u64, zero: bool) {
    let config = match zero {
        true => DEFAULT_CONFIG,
        false => OPTIMIZED_CONFIG,
    };

    let options = CMAESOptions::custom(Config::dimensions())
        .threads(1)
        .initial_mean(
            config
                .to_vec()
                .iter()
                .map(|p| (*p as f64))
                .collect(),
        )
        .initial_standard_deviations(
            config
                .to_vec()
                .iter()
                .map(|p| (*p * 0.05 + 1.) as f64)
                .collect(),
        )
        .initial_step_size(10.)
        // .stable_generations(250., 50)
        .max_evaluations(num_batches as usize);

    let solution = cmaes_loop(&FitnessEvaluator, options).unwrap();
    let new_config = Config::from_vec(solution.0.iter().map(|p| (*p as f32)).collect());
    let avg_score = -solution.1;

    println!("New average score: {}", avg_score);
    println!("{:?}", new_config);
}
