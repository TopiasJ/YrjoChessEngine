use rand::Rng;
use crate::chromosome::{ChromosomeRepository, init_new_chromosomes, Chromosome};

pub fn tournament<REPO: ChromosomeRepository>(wanted_chromosome_count: i32, depth: i32, old_chromosomes_repository: &REPO) {
    let old_boys = old_chromosomes_repository.read_chromosomes().unwrap(); // todo handle errors
    let old_chromosomes_count = old_boys.len() as i32;
    println!("Amount of dudes before tournament: {}", old_chromosomes_count);

    let mut players = init_new_chromosomes(wanted_chromosome_count-old_chromosomes_count, 0.3);
    players.extend(old_boys);

    let matches = randomize_opponents(players);
    for aMatch in matches {
        play_best_of_3_match(aMatch, depth);
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

    //let mut score = play_chess_match(players.0, players.1, depth);
}
/*
fn play_chess_match(player1: Chromosome, player2: Chromosome, depth: i32) -> i32 {
    let mut board = Board::default();
    let mut game = Game::new(board, player1, player2, depth);
    game.play();
    game.get_score()
}*/