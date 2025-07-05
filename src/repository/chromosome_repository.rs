use crate::chromosome::Chromosome;

pub trait ChromosomeRepository {
    fn read_chromosomes(&self) -> Result<Vec<Chromosome>, String>;
    #[allow(dead_code)]
    fn write_chromosomes(&mut self, chromosomes: &[Chromosome]) -> Result<(), String>;
}

pub struct MemoryChromosomeRepository {
    chromosomes: Vec<Chromosome>,
}

impl MemoryChromosomeRepository {
    pub fn new() -> Self {
        Self {
            chromosomes: Vec::new(),
        }
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