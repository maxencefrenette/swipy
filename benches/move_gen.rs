#[macro_use]
extern crate criterion;
extern crate swipy_engine;

use criterion::Criterion;
use swipy_engine::Board;

fn gen_moves(c: &mut Criterion) {
    let empty_board = Board::from_u64(0x0000_0100_0000_0000);
    c.bench_function("gen_moves (empty board)", move |b| {
        b.iter(|| empty_board.gen_moves())
    });

    let full_board = Board::from_u64(0xBA92_7621_0221_1001);
    c.bench_function("gen_moves (full board)", move |b| {
        b.iter(|| full_board.gen_moves())
    });
}

fn gen_tile_spawns(c: &mut Criterion) {
    let empty_board = Board::from_u64(0x0000_0100_0000_0000);
    c.bench_function("gen_tile_spawns (empty board)", move |b| {
        b.iter(|| empty_board.gen_tile_spawns())
    });

    let full_board = Board::from_u64(0xBA92_7621_0221_1001);
    c.bench_function("gen_tile_spawns (full board)", move |b| {
        b.iter(|| full_board.gen_tile_spawns())
    });
}

criterion_group!(benches, gen_moves, gen_tile_spawns);
criterion_main!(benches);
