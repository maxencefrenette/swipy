use engine::Engine;
use game::Board;
use v_function::VFunction;

pub fn train_td<F>(weights: F::Weights, num_batches: &u64, alpha: &f32) -> F::Weights
where
    F: VFunction,
{
    let mut engine = Engine::<F>::new(weights);

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

            if delta < -5. {
                println!("{} {} {} {}", r, next_eval, eval, delta);
            }

            engine.learn(&state, &delta);

            state = next_state;
        }

        println!("Game {}, Score: {}", i, state.score());
    }

    engine.into_weights()
}
