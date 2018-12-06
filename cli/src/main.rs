#[macro_use]
extern crate clap;
extern crate indicatif;
extern crate serde_json;
extern crate statistical;
extern crate swipy_engine;

mod cli_helpers;

use clap::{App, AppSettings, Arg, SubCommand};
use cli_helpers::{parse_arg, OutputFormat, VFunctionChoice};
use indicatif::{ProgressBar, ProgressStyle};
use statistical::{mean, standard_deviation, univariate::standard_error_mean};
use swipy_engine::{
    train_td,
    v_function::{Legacy, LegacyWeights, NTupleSmall, NTupleSmallWeights, VFunction, Weights},
    Board, Engine,
};

const DEFAULT_DEPTH: &str = "3";
const DEFAULT_LEARNING_RATE: &str = "0.0005";

fn init_clap<'a, 'b>() -> App<'a, 'b> {
    let v_function = Arg::with_name("v_function")
        .long("v_function")
        .takes_value(true)
        .default_value("legacy")
        .possible_values(VFunctionChoice::possible_values())
        .help("The V-function that will be trained and used");

    let format = Arg::with_name("format")
        .long("format")
        .takes_value(true)
        .default_value("human")
        .possible_values(OutputFormat::possible_values())
        .help("The format of the output");

    let play = SubCommand::with_name("play")
        .about("plays one game, logging the board to the command line")
        .arg(
            // TODO: Add a validator
            Arg::with_name("depth")
                .long("depth")
                .takes_value(true)
                .default_value(DEFAULT_DEPTH)
                .help("The expectimax search depth"),
        ).arg(&v_function);
    let bench = SubCommand::with_name("bench")
        .about("plays N games to test the strength of the AI")
        .arg(
            Arg::with_name("N")
                .help("The amount of games to play")
                .required(true)
                .takes_value(true),
        ).arg(
            // TODO: Add a validator
            Arg::with_name("depth")
                .long("depth")
                .takes_value(true)
                .default_value(DEFAULT_DEPTH)
                .help("The expectimax search depth"),
        ).arg(&v_function);
    let train = SubCommand::with_name("train")
        .about("continuously plays to optimize the AI")
        .arg(
            Arg::with_name("zero")
                .short("z")
                .help("Starts training from scratch"),
        ).arg(
            // TODO: Add a validator
            Arg::with_name("alpha")
                .long("alpha")
                .takes_value(true)
                .default_value(DEFAULT_LEARNING_RATE)
                .help("The learning rate"),
        ).arg(
            Arg::with_name("N")
                .help("The amount of batches of 5 games to play")
                .required(true)
                .takes_value(true),
        ).arg(&v_function)
        .arg(&format);

    App::new("Swipy - 2048 AI")
        .author(crate_authors!(", "))
        .version(crate_version!())
        .about("A 2048 AI")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommands(vec![play, bench, train])
}

fn main() {
    let app = init_clap();
    let matches = app.get_matches();

    match matches.subcommand_name().unwrap() {
        "play" => {
            let subcommand_matches = matches.subcommand_matches("play").unwrap();
            let depth = parse_arg::<u8>(subcommand_matches, "depth");
            let v_function = parse_arg::<VFunctionChoice>(subcommand_matches, "v_function");

            match v_function {
                VFunctionChoice::Legacy => play(
                    &mut Engine::<Legacy>::new(LegacyWeights::optimized()),
                    depth,
                ),
                VFunctionChoice::NTupleSmall => play(
                    &mut Engine::<NTupleSmall>::new(NTupleSmallWeights::optimized()),
                    depth,
                ),
            };
        }
        "bench" => {
            let subcommand_matches = matches.subcommand_matches("bench").unwrap();
            let num_games = parse_arg::<u64>(subcommand_matches, "N");
            let depth = parse_arg::<u8>(subcommand_matches, "depth");
            let v_function = parse_arg::<VFunctionChoice>(subcommand_matches, "v_function");

            match v_function {
                VFunctionChoice::Legacy => bench(
                    &mut Engine::<Legacy>::new(LegacyWeights::optimized()),
                    num_games,
                    depth,
                ),
                VFunctionChoice::NTupleSmall => bench(
                    &mut Engine::<NTupleSmall>::new(NTupleSmallWeights::optimized()),
                    num_games,
                    depth,
                ),
            };
        }
        "train" => {
            let subcommand_matches = matches.subcommand_matches("train").unwrap();
            let num_batches = parse_arg::<u64>(subcommand_matches, "N");
            let zero = subcommand_matches.is_present("zero");
            let alpha = parse_arg::<f32>(subcommand_matches, "alpha");
            let v_function = parse_arg::<VFunctionChoice>(subcommand_matches, "v_function");
            let format = parse_arg::<OutputFormat>(subcommand_matches, "format");

            match v_function {
                VFunctionChoice::Legacy => train::<Legacy>(num_batches, alpha, zero, format),
                VFunctionChoice::NTupleSmall => {
                    train::<NTupleSmall>(num_batches, alpha, zero, format)
                }
            };
        }
        _ => unreachable!(),
    }
}

fn play_random_game(engine: &mut Engine<impl VFunction>, depth: u8, verbose: bool) -> Board {
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

fn play(engine: &mut Engine<impl VFunction>, depth: u8) {
    let board = play_random_game(engine, depth, true);
    println!("Final Score: {}", board.score());
}

fn bench(engine: &mut Engine<impl VFunction>, num_games: u64, depth: u8) {
    // Init engine
    let mut scores = Vec::with_capacity(num_games as usize);
    let mut tiles_reached = [0u64; 16];

    let play_games_bar = ProgressBar::new(num_games);
    play_games_bar.set_message("Playing games");
    play_games_bar.set_style(ProgressStyle::default_bar().template("{msg} {wide_bar} {eta}"));
    play_games_bar.tick();

    for _ in 0..num_games {
        let board = play_random_game(engine, depth, false);

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

fn train<F>(num_batches: u64, alpha: f32, zero: bool, format: OutputFormat)
where
    F: VFunction,
{
    let weights = match zero {
        true => F::Weights::default(),
        false => F::Weights::optimized(),
    };

    let mut engine = Engine::<F>::new(weights);

    train_td(&mut engine, &num_batches, &alpha, |progress| match format {
        OutputFormat::Human => {
            println!("Game {}, Average Score: {}", progress.game, progress.score)
        }
        OutputFormat::Json => println!("{}", serde_json::to_string(&progress).unwrap()),
    });

    let new_weights = engine.into_weights();

    match format {
        OutputFormat::Human => println!("{:?}", new_weights),
        OutputFormat::Json => println!("{}", serde_json::to_string(&new_weights).unwrap()),
    };
}
