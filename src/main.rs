#[macro_use]
extern crate clap;
extern crate cmaes;
extern crate statistical;
extern crate swipy_engine;

use clap::{App, AppSettings, Arg, SubCommand};
use cmaes::*;
use statistical::{mean, standard_deviation, univariate::standard_error_mean};
use swipy_engine::{Board, Config, Engine, DEFAULT_CONFIG, OPTIMIZED_CONFIG};

fn main() {
    let app = init_clap();
    let matches = app.get_matches();

    match matches.subcommand_name().unwrap() {
        "play" => {
            let mut engine = Engine::new(OPTIMIZED_CONFIG);
            let score = play_random_game(&mut engine, true);
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
            let mut engine = Engine::new(OPTIMIZED_CONFIG);

            for i in 0..num_games {
                if i % 5 == 0 && i != 0 {
                    println!("{}/{}", i, num_games);
                }

                scores.push(play_random_game(&mut engine, false));
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

fn play_random_game(engine: &mut Engine, verbose: bool) -> f32 {
    let mut board = Board::new();

    if verbose {
        println!("{:?}", board);
        println!();
    }

    while !board.is_dead() {
        let mov = engine.search(board, 2.);
        board = board.make_move(&mov);

        if verbose {
            println!("{:?}", board);
            println!();
        }
    }

    board.score()
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
            score += play_random_game(&mut engine, false) as f64;
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
