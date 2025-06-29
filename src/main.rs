mod alpha_beta_algorithm;
mod chromosome;
mod evaluator;
mod tests;
mod visualizer;
mod tournament;

use crate::evaluator::Evaluator;
use crate::{alpha_beta_algorithm::AlgorithmTraits, visualizer::Visualizer};
use alpha_beta_algorithm::AlphaBetaAlgorithm;
use chess::{Board, BoardStatus, Color};
use clap::Parser;
use std::str::FromStr;

#[derive(Parser)]
#[command(name = "YrjoChessEngine")]
#[command(about = "A Rust-based chess engine with Alpha-Beta algorithm")]
#[command(version)]
struct Args {
    /// Calculation depth (half-moves). Higher values = stronger play but slower
    #[arg(short = 'd', long, default_value = "5")]
    depth: i32,

    /// Starting position in FEN notation. Default is standard chess position
    #[arg(short = 'f', long)]
    fen: Option<String>,

    /// Maximum number of moves before forcing a draw
    #[arg(short = 'm', long, default_value = "200")]
    max_moves: u32,

    /// Show evaluation scores after each move
    #[arg(short = 'e', long, default_value = "true")]
    show_eval: bool,

    /// Show board visualization after each move
    #[arg(short = 'b', long, default_value = "true")]
    show_board: bool,

    /// Delay between moves in milliseconds (for visualization)
    #[arg(short = 'l', long, default_value = "0")]
    delay: u64,
}

fn main() {
    let args = Args::parse();

    // Validate depth
    if args.depth < 1 || args.depth > 10 {
        eprintln!("Error: Depth must be between 1 and 10");
        std::process::exit(1);
    }

    // Parse starting position
    let mut board = if let Some(ref fen) = args.fen {
        match Board::from_str(fen) {
            Ok(b) => b,
            Err(_) => {
                eprintln!("Error: Invalid FEN position");
                std::process::exit(1);
            }
        }
    } else {
        Board::default()
    };

    println!("YrjoChessEngine v{}", env!("CARGO_PKG_VERSION"));
    println!("Starting with depth: {}", args.depth);
    if let Some(ref fen) = args.fen {
        println!("Custom position: {}", fen);
    }
    println!("Max moves: {}", args.max_moves);
    println!("---");

    let mut move_amount = 0;
    let mut alg = AlphaBetaAlgorithm;
    
    loop {
        let ai_move = match alg.get_best_move(board, args.depth) {
            Some(mv) => mv,
            None => {
                println!("No legal moves available!");
                break;
            }
        };
        
        board = board.make_move_new(ai_move);
        move_amount += 1;

        if args.show_eval {
            let eval = Evaluator::evaluate(board);
            println!("Move {}: Evaluation = {}", move_amount, eval);
        }

        if args.show_board {
            Visualizer::visualize_board(board);
            println!();
        }

        // Add delay if specified
        if args.delay > 0 {
            std::thread::sleep(std::time::Duration::from_millis(args.delay));
        }

        if move_amount >= args.max_moves {
            println!("Maximum moves reached ({}). Forcing draw.", args.max_moves);
            break;
        }

        let game_result: BoardStatus = board.status();

        if game_result != BoardStatus::Ongoing {
            println!("Game ended after {} moves", move_amount);
            match game_result {
                BoardStatus::Checkmate => {
                    let winner = match board.side_to_move() {
                        Color::Black => "White",
                        Color::White => "Black",
                    };
                    println!("Checkmate: {} wins!", winner);
                }
                BoardStatus::Stalemate => {
                    println!("Stalemate - Draw");
                }
                _ => {
                    println!("Game ended - Draw");
                }
            }
            break;
        }
    }
}
