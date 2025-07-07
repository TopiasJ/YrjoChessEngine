mod alpha_beta_algorithm;
mod benchmark;
mod chromosome;
mod evaluator;
mod repository;
mod tests;
mod tournament;
mod visualizer;

use crate::benchmark::BenchmarkRunner;
use crate::evaluator::Evaluator;
use crate::repository::{FileChromosomeRepository, MemoryChromosomeRepository};
use crate::tournament::tournament;
use crate::{alpha_beta_algorithm::AlgorithmTraits, visualizer::Visualizer};
use alpha_beta_algorithm::AlphaBetaAlgorithm;
use chess::{Board, BoardStatus, Color};
use clap::{Parser, Subcommand};
use std::str::FromStr;

#[derive(Parser)]
#[command(name = "YrjoChessEngine")]
#[command(about = "A Rust-based chess engine with Alpha-Beta algorithm")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Play a single game
    Single(SingleArgs),
    /// Run a tournament
    Tournament(TournamentArgs),
    /// Run performance benchmarks
    Benchmark(BenchmarkArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct SingleArgs {
    /// Calculation depth (half-moves). Higher values = stronger play but slower
    #[arg(short = 'd', long, default_value = "5")]
    pub depth: i32,

    /// Starting position in FEN notation. Default is standard chess position
    #[arg(short = 'f', long)]
    pub fen: Option<String>,

    /// Maximum number of moves before forcing a draw
    #[arg(short = 'm', long, default_value = "200")]
    pub max_moves: u32,

    /// Show evaluation scores after each move
    #[arg(short = 'e', long, default_value = "true")]
    pub show_eval: bool,

    /// Show board visualization after each move
    #[arg(short = 'b', long, default_value = "true")]
    pub show_board: bool,

    /// Delay between moves in milliseconds (for visualization)
    #[arg(short = 'l', long, default_value = "0")]
    pub delay: u64,
}

#[derive(Parser, Debug, Clone)]
pub struct TournamentArgs {
    /// Number of chromosomes to use in the tournament
    #[arg(short = 'n', long, default_value = "10")]
    pub wanted_chromosome_count: i32,
    /// Calculation depth (half-moves)
    #[arg(short = 'd', long, default_value = "5")]
    pub depth: i32,
    /// File path for persistent chromosome storage (optional, uses memory if not provided)
    #[arg(short = 'f', long)]
    pub file_path: Option<String>,
    /// Number of tournaments to run (0 or not specified = infinite)
    #[arg(short = 't', long, default_value = "0")]
    pub tournament_count: u32,
}

#[derive(Parser, Debug, Clone)]
pub struct BenchmarkArgs {
    /// Calculation depth (half-moves) for benchmark
    #[arg(short = 'd', long, default_value = "4")]
    pub depth: i32,
    /// Custom FEN position to benchmark (optional, uses standard positions if not provided)
    #[arg(short = 'f', long)]
    pub fen: Option<String>,
    /// Number of benchmark iterations to run for averaging
    #[arg(short = 'i', long, default_value = "1")]
    pub iterations: u32,
}

fn run_single_game(args: &SingleArgs) {
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
        println!("Custom position: {fen}");
    }
    println!("Max moves: {}", args.max_moves);
    println!("---");

    let mut move_amount = 0;
    let mut alg = AlphaBetaAlgorithm::new();
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
            println!("Move {move_amount}: Evaluation = {eval}");
        }
        if args.show_board {
            Visualizer::visualize_board(board);
            println!();
        }
        if args.delay > 0 {
            std::thread::sleep(std::time::Duration::from_millis(args.delay));
        }
        if move_amount >= args.max_moves {
            println!("Maximum moves reached ({}). Forcing draw.", args.max_moves);
            break;
        }
        let game_result: BoardStatus = board.status();
        if game_result != BoardStatus::Ongoing {
            println!("Game ended after {move_amount} moves");
            match game_result {
                BoardStatus::Checkmate => {
                    let winner = match board.side_to_move() {
                        Color::Black => "White",
                        Color::White => "Black",
                    };
                    println!("Checkmate: {winner} wins!");
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

fn run_benchmark(args: &BenchmarkArgs) {
    let mut runner = BenchmarkRunner::new();
    
    if let Some(fen) = &args.fen {
        // Benchmark a specific position
        println!("Benchmarking custom position at depth {}", args.depth);
        
        for i in 1..=args.iterations {
            if args.iterations > 1 {
                println!("Iteration {}/{}", i, args.iterations);
            }
            
            match runner.benchmark_position("Custom Position", fen, args.depth) {
                Ok(result) => {
                    println!("{}: {} nodes/sec ({} nodes in {} ms)", 
                            result.position_name, 
                            result.nodes_per_second as u64, 
                            result.nodes_searched, 
                            result.time_ms);
                }
                Err(e) => {
                    eprintln!("Error benchmarking position: {}", e);
                    std::process::exit(1);
                }
            }
        }
    } else {
        // Run standard benchmark suite
        for i in 1..=args.iterations {
            if args.iterations > 1 {
                println!("\nBenchmark iteration {}/{}", i, args.iterations);
            }
            
            if let Err(e) = runner.run_standard_benchmark(args.depth) {
                eprintln!("Error running benchmark: {}", e);
                std::process::exit(1);
            }
            
            if i < args.iterations {
                runner.results.clear(); // Clear results for next iteration
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Single(args) => run_single_game(args),
        Commands::Tournament(args) => {
            if let Some(file_path) = &args.file_path {
                // Use file-based repository
                match FileChromosomeRepository::new(file_path) {
                    Ok(mut repo) => {
                        println!("Using file repository: {file_path}");
                        tournament(args.wanted_chromosome_count, args.depth, args.tournament_count, &mut repo);
                    }
                    Err(e) => {
                        eprintln!("Error creating file repository: {e}");
                        std::process::exit(1);
                    }
                }
            } else {
                // Use memory repository
                println!("Using memory repository (chromosomes will not persist)");
                let mut repo = MemoryChromosomeRepository::new();
                tournament(args.wanted_chromosome_count, args.depth, args.tournament_count, &mut repo);
            }
        }
        Commands::Benchmark(args) => run_benchmark(args),
    }
}
