use crate::chromosome::{ChromosomeRepository, init_new_chromosomes};

fn tournament<REPO: ChromosomeRepository>(wanted_chromosome_count: i32, depth: i32, old_chromosomes_repository: &REPO) {
    let old_boys = old_chromosomes_repository.read_chromosomes().unwrap(); // todo handle errors
    let old_chromosomes_count = old_boys.len() as i32;
    println!("Amount of dudes before tournament: {}", old_chromosomes_count);

    let mut players = init_new_chromosomes(wanted_chromosome_count-old_chromosomes_count, 0.3);
    players.extend(old_boys)
}
