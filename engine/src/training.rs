use engine::Engine;
use game::Board;
use v_function::VFunction;

pub fn train_td<F>(weights: F::Weights, num_batches: &u64, alpha: &f32) -> F::Weights
where
    F: VFunction,
{
    let mut engine = Engine::<F>::new(weights);

    for i in 0..*num_batches {
        let mut board = Board::new();

        while !board.is_dead() {
            let chosen_move = engine.search(board, 1);
            let new_board = board.make_move(&chosen_move);

            let r = new_board.score() - board.score();
            let old_eval = engine.static_eval(board);
            let new_eval = if new_board.is_dead() {
                new_board.score()
            } else {
                engine.static_eval(new_board)
            };

            let delta = alpha * (r + new_eval - old_eval);

            engine.learn(&board, &delta);

            board = new_board;
        }

        println!("Game {}, Score: {}", i, board.score());
    }

    engine.into_weights()
}
