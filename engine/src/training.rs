use crate::engine::Engine;
use crate::game::Board;
use crate::testing::benchmark;
use crate::v_function::VFunction;
use serde_derive::{Deserialize, Serialize};

pub fn train_td<F>(
    engine: &mut Engine<impl VFunction>,
    num_batches: u64,
    alpha: f32,
    benchmark_interval: u64,
    on_progress: F,
) where
    F: Fn(TrainingProgress) -> (),
{
    let mut score_acc: f32 = 0.;

    for i in 0..num_batches {
        // Report training stats
        if i % benchmark_interval == 0 {
            let training_score = if i == 0 {
                None
            } else {
                Some(score_acc / (benchmark_interval as f32))
            };
            let test_score = benchmark(engine, 25, 3, |_| ()).average;

            on_progress(TrainingProgress {
                game: i,
                training_score,
                test_score,
            });
            score_acc = 0.;
        }

        let mut state = Board::new_random();

        while !state.is_dead() {
            // Afterstate learning algorithm from Szubert and Jaśkowski
            let action = engine.search(state, 1);
            let afterstate = state.move_candidate(action);
            let next_state = state.make_move(action);

            let eval = engine.static_eval(afterstate);

            if next_state.is_dead() {
                let delta = alpha * -eval;
                engine.learn(state, delta);
                break;
            }

            let next_action = engine.search(next_state, 1);
            let next_afterstate = next_state.move_candidate(next_action);

            let r = next_afterstate.score() - afterstate.score();
            let next_eval = engine.static_eval(next_afterstate);

            let delta = alpha * (r + next_eval - eval);

            engine.learn(afterstate, delta);

            state = next_state;
        }

        score_acc += state.score();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrainingProgress {
    pub game: u64,
    pub training_score: Option<f32>,
    pub test_score: f32,
}
