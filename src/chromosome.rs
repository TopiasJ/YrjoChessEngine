use rand::Rng;

// Default piece values for chess evaluation
const DEFAULT_PAWN_VALUE: i32 = 100;
const DEFAULT_KNIGHT_VALUE: i32 = 300;
const DEFAULT_BISHOP_VALUE: i32 = 300;
const DEFAULT_ROOK_VALUE: i32 = 500;
const DEFAULT_QUEEN_VALUE: i32 = 900;
const DEFAULT_KING_VALUE: i32 = 10000;

#[derive(Clone, Debug)]
pub struct Chromosome {
    pub pawn_value: i32,
    pub knight_value: i32,
    pub bishop_value: i32,
    pub rook_value: i32,
    pub queen_value: i32,
    pub king_value: i32,
}

impl Chromosome {
    #[allow(dead_code)]
    pub fn new_default() -> Self {
        Self {
            pawn_value: DEFAULT_PAWN_VALUE,
            knight_value: DEFAULT_KNIGHT_VALUE,
            bishop_value: DEFAULT_BISHOP_VALUE,
            rook_value: DEFAULT_ROOK_VALUE,
            queen_value: DEFAULT_QUEEN_VALUE,
            king_value: DEFAULT_KING_VALUE,
        }
    }

    pub fn new_random(variance: f32) -> Self {
        let mut rng = rand::rng();
        Self {
            pawn_value: DEFAULT_PAWN_VALUE,
            knight_value: rng.random_range(
                (DEFAULT_KNIGHT_VALUE as f32 * (1.0 - variance)) as i32..(DEFAULT_KNIGHT_VALUE as f32 * (1.0 + variance)) as i32
            ),
            bishop_value: rng.random_range(
                (DEFAULT_BISHOP_VALUE as f32 * (1.0 - variance)) as i32..(DEFAULT_BISHOP_VALUE as f32 * (1.0 + variance)) as i32
            ),
            rook_value: rng.random_range(
                (DEFAULT_ROOK_VALUE as f32 * (1.0 - variance)) as i32..(DEFAULT_ROOK_VALUE as f32 * (1.0 + variance)) as i32
            ),
            queen_value: rng.random_range(
                (DEFAULT_QUEEN_VALUE as f32 * (1.0 - variance)) as i32..(DEFAULT_QUEEN_VALUE as f32 * (1.0 + variance)) as i32
            ),
            king_value: DEFAULT_KING_VALUE
        }
    }

    // switch 2 piece values between chromosomes
    #[allow(dead_code)]
    pub fn crossover(&mut self, other: &mut Self) {
        let mut rng = rand::rng();
        let what_to_cross = rng.random_range(0..=4);
        match what_to_cross {
            1 => {
                println!("doing crossover for knight. Original value gene1: {}, gene2: {}", self.knight_value, other.knight_value);
                std::mem::swap(&mut self.knight_value, &mut other.knight_value);
                println!("New value gene1: {}, gene2: {}", self.knight_value, other.knight_value);
            }
            2 => {
                println!("doing crossover for bishop. Original value gene1: {}, gene2: {}", self.bishop_value, other.bishop_value);
                std::mem::swap(&mut self.bishop_value, &mut other.bishop_value);
                println!("New value gene1: {}, gene2: {}", self.bishop_value, other.bishop_value);
            }
            3 => {
                println!("doing crossover for rook. Original value gene1: {}, gene2: {}", self.rook_value, other.rook_value);
                std::mem::swap(&mut self.rook_value, &mut other.rook_value);
                println!("New value gene1: {}, gene2: {}", self.rook_value, other.rook_value);
            }
            4 => {
                println!("doing crossover for queen. Original value gene1: {}, gene2: {}", self.queen_value, other.queen_value);
                std::mem::swap(&mut self.queen_value, &mut other.queen_value);
                println!("New value gene1: {}, gene2: {}", self.queen_value, other.queen_value);
            }
            _ => {}
        }
    }

    #[allow(dead_code)]
    pub fn mutation(&mut self, mutation_chance: f32, variance: f32) {
        let mut rng = rand::rng();
        let rando = rng.random_range(0.0..=100.0);
        let do_mutation = rando <= mutation_chance * 100.0;

        if do_mutation {
            let what_to_mutate = rng.random_range(1..=4);
            match what_to_mutate {
                1 => {
                    let original_value = self.knight_value;
                    self.knight_value = rng.random_range(
                        original_value - (original_value as f32 * variance) as i32..original_value + (original_value as f32 * variance) as i32
                    );
                }
                2 => {
                    let original_value = self.bishop_value;
                    self.bishop_value = rng.random_range(
                        original_value - (original_value as f32 * variance) as i32..original_value + (original_value as f32 * variance) as i32
                    );
                }
                3 => {
                    let original_value = self.rook_value;
                    self.rook_value = rng.random_range(
                        original_value - (original_value as f32 * variance) as i32..original_value + (original_value as f32 * variance) as i32
                    );
                }
                4 => {
                    let original_value = self.queen_value;
                    self.queen_value = rng.random_range(
                        original_value - (original_value as f32 * variance) as i32..original_value + (original_value as f32 * variance) as i32
                    );
                }
                _ => {}
            }
            println!("Mutation applied to chromosome");
        }
    }
}


pub fn init_new_chromosomes(amount: i32, variance: f32) -> Vec<Chromosome> {
    let mut chromosomes = Vec::new();
    for _ in 0..amount {
        let chromosome = Chromosome::new_random(variance);
        chromosomes.push(chromosome);
    }
    chromosomes
}