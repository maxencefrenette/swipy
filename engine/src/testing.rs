use crate::engine::Engine;
use crate::game::Board;
use crate::v_function::VFunction;
use serde_derive::{Deserialize, Serialize};
use statistical::{mean, standard_deviation, univariate::standard_error_mean};

pub fn play_random_game(engine: &mut Engine<impl VFunction>, depth: u8, verbose: bool) -> Board {
    let mut board = Board::new_random();

    if verbose {
        println!("{:?}", board);
        println!();
    }

    while !board.is_dead() {
        let mov = engine.search(board, depth);
        board = board.make_move(mov);

        if verbose {
            println!("{:?}", board);
            println!();
        }
    }

    board
}

pub fn benchmark<F>(
    engine: &mut Engine<impl VFunction>,
    num_games: u64,
    depth: u8,
    on_progress: F,
) -> BenchmarkResult
where
    F: Fn(u64) -> (),
{
    let mut scores = Vec::with_capacity(num_games as usize);
    let mut tiles_reached_count = [0u64; 16];

    for i in 0..num_games {
        let board = play_random_game(engine, depth, false);

        scores.push(board.score());

        for j in 0..=board.highest_tile() {
            tiles_reached_count[j as usize] += 1;
        }

        on_progress(i);
        engine.reset();
    }

    let average = mean(scores.as_slice());
    let sd = standard_deviation(scores.as_slice(), Some(average));
    let error = standard_error_mean(sd, scores.len() as f32, None);
    let lower_bound = average - 1.96 * error;
    let upper_bound = average + 1.96 * error;

    let mut tiles_reached = [0.; 16];
    for i in 0..16 {
        tiles_reached[i] = (tiles_reached_count[i] as f32) / (num_games as f32);
    }

    BenchmarkResult {
        average,
        standard_deviation: sd,
        error,
        lower_bound,
        upper_bound,
        tiles_reached,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub average: f32,
    pub standard_deviation: f32,
    pub error: f32,
    pub lower_bound: f32,
    pub upper_bound: f32,
    pub tiles_reached: [f32; 16],
}
