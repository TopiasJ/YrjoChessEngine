use chess::{Board, BoardStatus};
use rand::Rng;
use crate::alpha_beta_algorithm::{AlphaBetaAlgorithm, AlgorithmTraits};
use crate::chromosome::{init_new_chromosomes, Chromosome};
use crate::repository::ChromosomeRepository;

pub fn tournament<REPO: ChromosomeRepository>(wanted_chromosome_count: i32, depth: i32, old_chromosomes_repository: &mut REPO) {
    let old_boys = old_chromosomes_repository.read_chromosomes().unwrap(); // todo handle errors
    let old_chromosomes_count = old_boys.len() as i32;
    println!("Amount of dudes before tournament: {old_chromosomes_count}");

    let mut players = init_new_chromosomes(wanted_chromosome_count-old_chromosomes_count, 0.3);
    players.extend(old_boys);

    let mut current_round_players = players;
    let mut round_number = 1;
    
    // Keep playing rounds until we have fewer than 2 players or reach max 2 rounds
    while current_round_players.len() >= 2 && round_number <= 2 {
        println!("\n=== ROUND {round_number} ===");
        println!("Players in this round: {}", current_round_players.len());
        
        let matches = randomize_opponents(current_round_players);
        let mut winners = Vec::new();
        
        for a_match in matches {
            let winner = play_best_of_3_match(a_match, depth);
            winners.push(winner);
        }
        
        // If we have winners, do crossover and mutation
        if winners.len() > 1 && round_number < 2 {
            println!("\n=== CROSSOVER AND MUTATION ===");
            let new_generation = do_crossover_and_mutation(winners, wanted_chromosome_count);
            current_round_players = new_generation;
        } else {
            // Only one winner or reached max rounds - tournament complete
            current_round_players = winners;
        }
        
        round_number += 1;
    }
    
    if let Some(champion) = current_round_players.first() {
        println!("\n🏆 TOURNAMENT CHAMPION 🏆");
        println!("Champion: {champion:?}");
        
        // Save all evolved chromosomes to repository
        if let Err(e) = old_chromosomes_repository.write_chromosomes(&current_round_players) {
            eprintln!("Failed to save tournament results: {e}");
        } else {
            println!("Tournament results saved to repository!");
        }
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
fn play_best_of_3_match(players:(Chromosome,Chromosome), depth: i32) -> Chromosome {
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
            return players.0;
        }
        if player2_wins == 2 {
            println!("Player 2 wins the match 2-{player1_wins}");
            return players.1;
        }
    }
    
    // Handle tie case - return player with better overall stats (or random)
    if player1_wins == player2_wins {
        println!("Match tied {player1_wins}-{player2_wins}");
        // In case of tie, randomly select winner
        let mut rng = rand::rng();
        if rng.random_range(0..2) == 0 {
            println!("Player 1 wins by tiebreaker");
            players.0
        } else {
            println!("Player 2 wins by tiebreaker");
            players.1
        }
    } else if player1_wins > player2_wins {
        players.0
    } else {
        players.1
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
            // Use default evaluator to determine winner based on material
            let evaluation = crate::evaluator::Evaluator::evaluate(board);
            if evaluation > 0 {
                return 1; // White (player1) wins
            } else if evaluation < 0 {
                return -1; // Black (player2) wins
            } else {
                return 0; // Draw (equal material)
            }
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

fn do_crossover_and_mutation(winner_genes: Vec<Chromosome>, wanted_genes_count: i32) -> Vec<Chromosome> {
    let mut new_generation = Vec::new();
    let mut current_gene_count = 0;
    
    // First, do crossover for pairs of winners
    let mut previous: Option<Chromosome> = None;
    
    for gene in winner_genes.iter().cloned() {
        if previous.is_none() {
            previous = Some(gene);
        } else if let Some(mut prev_gene) = previous.take() {
            let mut current_gene = gene;
            
            // Do crossover between the two genes
            prev_gene.crossover(&mut current_gene);
            
            // Apply mutation to both
            prev_gene.mutation(0.1, 0.2); // 10% chance, 20% variance
            current_gene.mutation(0.1, 0.2);
            
            // Save both genes
            new_generation.push(prev_gene);
            new_generation.push(current_gene);
            current_gene_count += 2;
            
            previous = None;
        }
    }
    
    // If we have a leftover gene (odd number of winners), add it to the new generation
    if let Some(leftover) = previous {
        new_generation.push(leftover);
        current_gene_count += 1;
    }
    
    // Fill up to wanted count by crossing over random pairs from winners
    while current_gene_count < wanted_genes_count {
        // Shuffle winners and pick first two
        let mut rng = rand::rng();
        let len = winner_genes.len();
        if len >= 2 {
            let idx1 = rng.random_range(0..len);
            let mut idx2 = rng.random_range(0..len);
            while idx2 == idx1 {
                idx2 = rng.random_range(0..len);
            }
            
            let mut gene1 = winner_genes[idx1].clone();
            let mut gene2 = winner_genes[idx2].clone();
            
            // Do crossover
            gene1.crossover(&mut gene2);
            
            // Apply mutation
            gene1.mutation(0.1, 0.2);
            gene2.mutation(0.1, 0.2);
            
            // Save both genes
            new_generation.push(gene1);
            new_generation.push(gene2);
            current_gene_count += 2;
        } else {
            // Not enough winners, just duplicate the existing one
            if let Some(winner) = winner_genes.first() {
                let mut new_gene = winner.clone();
                new_gene.mutation(0.2, 0.3); // Higher mutation for diversity
                new_generation.push(new_gene);
                current_gene_count += 1;
            } else {
                break;
            }
        }
    }
    
    // Trim to exact count if we went over
    new_generation.truncate(wanted_genes_count as usize);
    
    println!("Generated {} new chromosomes from {} winners", new_generation.len(), winner_genes.len());
    
    new_generation
}
