use chess::{Board, BoardStatus};
use rand::Rng;
use crate::alpha_beta_algorithm::{AlphaBetaAlgorithm, AlgorithmTraits};
use crate::chromosome::{init_new_chromosomes, Chromosome};
use crate::repository::ChromosomeRepository;

pub fn tournament<REPO: ChromosomeRepository>(wanted_chromosome_count: i32, depth: i32, old_chromosomes_repository: &REPO) {
    let old_boys = old_chromosomes_repository.read_chromosomes().unwrap(); // todo handle errors
    let old_chromosomes_count = old_boys.len() as i32;
    println!("Amount of dudes before tournament: {old_chromosomes_count}");

    let mut players = init_new_chromosomes(wanted_chromosome_count-old_chromosomes_count, 0.3);
    players.extend(old_boys);

    let matches = randomize_opponents(players);
    for a_match in matches {
        play_best_of_3_match(a_match, depth);
    }
}
fn randomize_opponents(players: Vec<Chromosome>) -> Vec<(Chromosome, Chromosome)> {
    let mut matches = Vec::new();
    let mut players = players;
    while players.len() > 1 {
        let player1 = players.remove(rand::rng().random_range(0..players.len()));
        let player2 = players.remove(rand::rng().random_range(0..players.len()));
        matches.push((player1, player2));
    }
    matches
}
fn play_best_of_3_match(players:(Chromosome,Chromosome), depth: i32) {
    println!("Playing Best of 3 match");
    println!("Player 1: {:?}", players.0);
    println!("Player 2: {:?}", players.1);

    let mut player1_wins = 0;
    let mut player2_wins = 0;
    
    for game in 1..=3 {
        println!("Game {game}");
        let result = play_chess_match(players.0.clone(), players.1.clone(), depth);
        
        match result {
            1 => {
                player1_wins += 1;
                println!("Player 1 wins game {game}");
            }
            -1 => {
                player2_wins += 1;
                println!("Player 2 wins game {game}");
            }
            0 => {
                println!("Game {game} is a draw");
            }
            _ => {}
        }
        
        // Early termination if someone already won the match
        if player1_wins == 2 {
            println!("Player 1 wins the match 2-{player2_wins}");
            break;
        }
        if player2_wins == 2 {
            println!("Player 2 wins the match 2-{player1_wins}");
            break;
        }
    }
    
    if player1_wins == player2_wins {
        println!("Match tied {player1_wins}-{player2_wins}");
    }
}

fn play_chess_match(player1: Chromosome, player2: Chromosome, depth: i32) -> i32 {
    let mut board = Board::default();
    let mut alg = AlphaBetaAlgorithm;
    let mut move_count = 0;
    let max_moves = 100; // Prevent infinite games
    
    loop {
        // Check if game is over
        let game_result = board.status();
        if game_result != BoardStatus::Ongoing {
            match game_result {
                BoardStatus::Checkmate => {
                    // Return based on who got checkmated
                    return match board.side_to_move() {
                        chess::Color::White => -1, // White lost (player1 if white)
                        chess::Color::Black => 1,  // Black lost (player2 if black)
                    };
                }
                BoardStatus::Stalemate => return 0, // Draw
                _ => return 0, // Other draw conditions
            }
        }
        
        // Prevent infinite games
        if move_count >= max_moves {
            return 0; // Draw
        }
        
        // Get move from appropriate player
        let chess_move = match board.side_to_move() {
            chess::Color::White => alg.get_best_move_with_chromosome(board, depth, &player1),
            chess::Color::Black => alg.get_best_move_with_chromosome(board, depth, &player2),
        };
        
        match chess_move {
            Some(mov) => {
                board = board.make_move_new(mov);
                move_count += 1;
            }
            None => {
                // No legal moves available - should not happen if status is Ongoing
                return 0;
            }
        }
    }
}