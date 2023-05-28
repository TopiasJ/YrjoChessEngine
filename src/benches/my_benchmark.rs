use chess::{Board, BoardStatus, Color};
use crate::alpha_beta_algorithm::{AlphaBetaAlgorithm, AlgorithmTraits};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}


fn checkmate4() {
    let board = Board::from_str("8/8/8/5k1K/6r1/8/8/8 w - - 10 6").unwrap(); //4:lla
    let required_moves = game_loop(board, 8);

}

fn game_loop(mut board:Board, calculation_half_depth:i32) -> i32 {
    let mut move_amount = 0;
    let mut alg = AlphaBetaAlgorithm;
    loop {
        let ai_move = alg.get_best_move(board, calculation_half_depth).unwrap();
        board = board.make_move_new(ai_move);
        move_amount += 1;


    let game_result: BoardStatus = board.status();

    if game_result != BoardStatus::Ongoing {
        println!("Game ended");
        if game_result == BoardStatus::Checkmate {
            let winner = match board.side_to_move() {
                Color::Black => "White",
                Color::White => "Black",
            };
            println!("Checkmate: {} wins", winner);
        } else {
            println!("Stalemate");
        }
        return move_amount
    }
    }
}



criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);