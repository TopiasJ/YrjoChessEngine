mod alpha_beta_algorithm;
mod evaluator;
mod tests;
mod visualizer;

use crate::evaluator::Evaluator;
use crate::{alpha_beta_algorithm::AlgorithmTraits, visualizer::Visualizer};
use alpha_beta_algorithm::AlphaBetaAlgorithm;
use chess::{Board, BoardStatus, Color};

const CALCULATION_HALF_DEPTH: i32 = 5;

fn main() {
    let mut board = Board::default();
    //let mut board = Board::from_str("7K/5k2/8/8/8/8/8/r7 w - - 0 1").unwrap();
    //let mut board = Board::from_str("7k/8/5K2/8/8/8/8/5R2 w - - 0 1").unwrap();
    //let mut board = Board::from_str("8/6K1/8/4k3/8/8/8/r7 w - - 0 1").unwrap(); // shakkimatti 8:lla
    //let mut board = Board::from_str("8/8/4k1K1/8/r7/8/8/8 b - - 7 4").unwrap(); //6:lla
    //let mut board = Board::from_str("8/8/4k1K1/8/6r1/8/8/8 w - - 8 5").unwrap(); //5:lla
    //let mut board = Board::from_str("8/8/8/5k1K/6r1/8/8/8 w - - 10 6").unwrap(); //4:lla

    let mut move_amount = 0;
    let mut alg = AlphaBetaAlgorithm;
    loop {
        let ai_move = alg.get_best_move(board, CALCULATION_HALF_DEPTH).unwrap();
        board = board.make_move_new(ai_move);
        move_amount += 1;
        //game.make_move(ai_move);

        //let start2 = Instant::now();
        let eva2 = Evaluator::evaluate(board);
        println!("eva2 {}", eva2); //,  start2.elapsed());

        Visualizer::visualize_board(board);
        //thread::sleep(time::Duration::from_millis(5));

        if move_amount > 200 {
            //  println!("forced draw");
            //    return;
        }
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
            return;
        }
    }
}
