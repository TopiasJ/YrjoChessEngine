mod visualizer;
mod evaluator;
mod alpha_beta_algorithm;

use std::str::FromStr;
use alpha_beta_algorithm::AlphaBetaAlgorithm;
use chess::{MoveGen, Board, EMPTY, ChessMove, BoardStatus, Color};
use rand::Rng;
use crate::{visualizer::Visualizer, alpha_beta_algorithm::AlgorithmTraits};
use crate::evaluator::Evaluator;

const CALCULATION_HALF_DEPTH :i32= 4; 

fn main() {
    // This structure is slow compared to using `Board` directly, so it is not recommended for engines.
    //let mut game = Game::new();
    let mut board = Board::default();
    //let mut board = Board::from_str("7K/5k2/8/8/8/8/8/r7 w - - 0 1").unwrap();
    //let mut board = Board::from_str("7k/8/5K2/8/8/8/8/5R2 w - - 0 1").unwrap();
    //let mut board = Board::from_str("8/6K1/8/4k3/8/8/8/r7 w - - 0 1").unwrap(); // shakkimatti 8:lla
    //let mut board = Board::from_str("8/8/4k1K1/8/r7/8/8/8 b - - 7 4").unwrap(); //6:lla
    //let mut board = Board::from_str("8/8/4k1K1/8/6r1/8/8/8 w - - 8 5").unwrap(); //5:lla
    //let mut board = Board::from_str("8/8/8/5k1K/6r1/8/8/8 w - - 10 6").unwrap(); //4:lla



    let mut move_amount = 0;
    loop {
        let mut alg = AlphaBetaAlgorithm;
        let ai_move = alg.get_best_move(board, CALCULATION_HALF_DEPTH).unwrap();
        board = board.make_move_new(ai_move);
        move_amount += 1;
        //game.make_move(ai_move);

        //let start2 = Instant::now();
        let eva2 = Evaluator::evaluate2(board);
        println!("eva2 {}",eva2);//,  start2.elapsed());

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
                let winner = match board.side_to_move(){
                    Color::Black => "White",
                    Color::White => "Black"
                };
                println!("Checkmate: {} wins",winner );
            }
            else {
                println!("Stalemate");
            }
            return;
        }
    }
}


fn _ai_get_first_move(board: &Board) -> Option<ChessMove> {
    // lets iterate over targets.
    let mut iterable = MoveGen::new_legal(board); //get all legal moves
    let targets = board.color_combined(!board.side_to_move());
    iterable.set_iterator_mask(*targets);

    for mov in &mut iterable {
        println!("move targets: {} {}", mov.get_source().get_rank().to_index(), mov.get_source().get_file().to_index());
        return Some(mov);
        // This move captures one of my opponents pieces (with the exception of en passant)
    }

    // now, iterate over the rest of the moves
    iterable.set_iterator_mask(!EMPTY);
    println!("remaining moves: {}", iterable.len());

    let die = rand::thread_rng().gen_range(0..iterable.len());
    let mut i = 0;
    for mov in &mut iterable {
        if die == i {
            println!("move: {} {}", mov.get_source().get_rank().to_index(), mov.get_source().get_file().to_index());
            return Some(mov);
        }
        i += 1;
        // This move does not capture anything
    }
    return None;
}