mod cli_helpers;

use crate::cli_helpers::{parse_arg, OutputFormat, VFunctionChoice};
use clap::{crate_authors, crate_version, App, AppSettings, Arg, SubCommand};
use indicatif::{ProgressBar, ProgressStyle};
use swipy_engine::{
    testing::{benchmark, play_random_game},
    train_td,
    v_function::{
        Legacy, LegacyWeights, NTupleMedium, NTupleMediumWeights, NTupleSmall, NTupleSmallWeights,
        VFunction, Weights,
    },
    Engine,
};

const DEFAULT_DEPTH: &str = "3";
const DEFAULT_LEARNING_RATE: &str = "0.0005";
const DEFAULT_BENCHMARK_INTERVAL: &str = "5000";

fn init_clap<'a, 'b>() -> App<'a, 'b> {
    let v_function = Arg::with_name("v_function")
        .long("v_function")
        .takes_value(true)
        .default_value("legacy")
        .possible_values(&VFunctionChoice::possible_values())
        .help("The V-function that will be trained and used");

    let format = Arg::with_name("format")
        .long("format")
        .takes_value(true)
        .default_value("human")
        .possible_values(&OutputFormat::possible_values())
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
        )
        .arg(&v_function);
    let bench = SubCommand::with_name("bench")
        .about("plays N games to test the strength of the AI")
        .arg(
            Arg::with_name("N")
                .help("The amount of games to play")
                .required(true)
                .takes_value(true),
        )
        .arg(
            // TODO: Add a validator
            Arg::with_name("depth")
                .long("depth")
                .takes_value(true)
                .default_value(DEFAULT_DEPTH)
                .help("The expectimax search depth"),
        )
        .arg(&v_function);
    let train = SubCommand::with_name("train")
        .about("continuously plays to optimize the AI")
        .arg(
            Arg::with_name("zero")
                .short("z")
                .help("Starts training from scratch"),
        )
        .arg(
            // TODO: Add a validator
            Arg::with_name("alpha")
                .long("alpha")
                .takes_value(true)
                .default_value(DEFAULT_LEARNING_RATE)
                .help("The learning rate"),
        )
        .arg(
            Arg::with_name("N")
                .help("The amount of batches of 5 games to play")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("benchmark-interval")
                .long("benchmark-interval")
                .help("The interval at which the current network should be tested")
                .required(false)
                .default_value(DEFAULT_BENCHMARK_INTERVAL)
                .takes_value(true),
        )
        .arg(&v_function)
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
                VFunctionChoice::NTupleMedium => play(
                    &mut Engine::<NTupleMedium>::new(NTupleMediumWeights::optimized()),
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
                VFunctionChoice::NTupleMedium => bench(
                    &mut Engine::<NTupleMedium>::new(NTupleMediumWeights::optimized()),
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
            let benchmark_interval = parse_arg::<u64>(subcommand_matches, "benchmark-interval");

            match v_function {
                VFunctionChoice::Legacy => {
                    train::<Legacy>(num_batches, alpha, zero, format, benchmark_interval)
                }
                VFunctionChoice::NTupleSmall => {
                    train::<NTupleSmall>(num_batches, alpha, zero, format, benchmark_interval)
                }
                VFunctionChoice::NTupleMedium => {
                    train::<NTupleMedium>(num_batches, alpha, zero, format, benchmark_interval)
                }
            };
        }
        _ => unreachable!(),
    }
}

fn play(engine: &mut Engine<impl VFunction>, depth: u8) {
    let board = play_random_game(engine, depth, true);
    println!("Final Score: {}", board.score());
}

fn bench(engine: &mut Engine<impl VFunction>, num_games: u64, depth: u8) {
    let play_games_bar = ProgressBar::new(num_games);
    play_games_bar.set_message("Playing games");
    play_games_bar.set_style(ProgressStyle::default_bar().template("{msg} {wide_bar} {eta}"));
    play_games_bar.tick();

    let results = benchmark(engine, num_games, depth, |_| play_games_bar.inc(1));

    play_games_bar.finish();
    println!();

    println!("{} games played.", num_games);
    println!(
        "Average score: {:.0} \u{00b1} {:.0}",
        results.average, results.error
    );
    println!("Standard deviation: {:.0}", results.standard_deviation);
    println!(
        "Confidence interval (95%): [{:.0}, {:.0}]",
        results.lower_bound, results.upper_bound
    );
    println!();

    for n in 8..13 {
        println!(
            "{}: {}%",
            u64::pow(2, n),
            results.tiles_reached[n as usize] * 100.
        );
    }
}

fn train<F>(num_batches: u64, alpha: f32, zero: bool, format: OutputFormat, benchmark_interval: u64)
where
    F: VFunction,
{
    let weights = if zero {
        F::Weights::default()
    } else {
        F::Weights::optimized()
    };

    let mut engine = Engine::<F>::new(weights);

    train_td(
        &mut engine,
        num_batches,
        alpha,
        benchmark_interval,
        |progress| match format {
            OutputFormat::Human => println!(
                "Game {}, Average Score: {}",
                progress.game, progress.test_score
            ),
            OutputFormat::Json => println!("{}", serde_json::to_string(&progress).unwrap()),
        },
    );

    let new_weights = engine.into_weights();

    match format {
        OutputFormat::Human => println!("{:?}", new_weights),
        OutputFormat::Json => println!("{}", serde_json::to_string(&new_weights).unwrap()),
    };
}
