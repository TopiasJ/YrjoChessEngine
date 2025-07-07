use crate::chromosome::Chromosome;

pub trait ChromosomeRepository {
    fn read_chromosomes(&self) -> Result<Vec<Chromosome>, String>;
    #[allow(dead_code)]
    fn write_chromosomes(&mut self, chromosomes: &[Chromosome]) -> Result<(), String>;

    fn write_tournament_winners(&mut self, winners: &[Chromosome]) -> Result<(), String> {
        self.write_chromosomes(winners)
    }

    fn write_tournament_winners_with_count(&mut self, winners: &[Chromosome], _player_count: i32) -> Result<(), String> {
        self.write_tournament_winners(winners)
    }

    fn validate_player_count(&self, _player_count: i32) -> Result<(), String> {
        Ok(()) // Default implementation - no validation for memory repositories
    }
}

pub struct MemoryChromosomeRepository {
    chromosomes: Vec<Chromosome>,
}

impl MemoryChromosomeRepository {
    pub fn new() -> Self {
        Self { chromosomes: Vec::new() }
    }
}

impl ChromosomeRepository for MemoryChromosomeRepository {
    fn read_chromosomes(&self) -> Result<Vec<Chromosome>, String> {
        Ok(self.chromosomes.clone())
    }

    fn write_chromosomes(&mut self, chromosomes: &[Chromosome]) -> Result<(), String> {
        self.chromosomes.extend(chromosomes.iter().cloned());
        Ok(())
    }
}
