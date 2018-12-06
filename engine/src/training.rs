use engine::Engine;
use game::Board;
use v_function::VFunction;

const REPORTING_INTERVAL: u64 = 1000;

pub fn train_td<F>(
    engine: &mut Engine<impl VFunction>,
    num_batches: &u64,
    alpha: &f32,
    on_progress: F,
) where
    F: Fn(TrainingProgress) -> (),
{
    let mut score_acc: f32 = 0.;

    for i in 0..*num_batches {
        let mut state = Board::new();

        while !state.is_dead() {
            // Afterstate learning algorithm from Szubert and Jaśkowski
            let action = engine.search(state, 1);
            let afterstate = state.move_candidate(&action);
            let next_state = state.make_move(&action);

            let eval = engine.static_eval(afterstate);

            if next_state.is_dead() {
                let delta = alpha * -eval;
                engine.learn(&state, &delta);
                break;
            }

            let next_action = engine.search(next_state, 1);
            let next_afterstate = next_state.move_candidate(&next_action);

            let r = next_afterstate.score() - afterstate.score();
            let next_eval = engine.static_eval(next_afterstate);

            let delta = alpha * (r + next_eval - eval);

            engine.learn(&state, &delta);

            state = next_state;
        }

        score_acc += state.score();

        if i % REPORTING_INTERVAL == 0 {
            let avg_score = score_acc / (REPORTING_INTERVAL as f32);
            on_progress(TrainingProgress::new(i, avg_score));
            score_acc = 0.;
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrainingProgress {
    pub game: u64,
    pub score: f32,
}

impl TrainingProgress {
    fn new(game: u64, score: f32) -> TrainingProgress {
        TrainingProgress { game, score }
    }
}
