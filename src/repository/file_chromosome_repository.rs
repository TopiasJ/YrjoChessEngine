use crate::chromosome::{Chromosome, TournamentHistory};
use crate::repository::ChromosomeRepository;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

/// Thread-safe file-based chromosome repository using RwLock.
/// 
/// Benefits of RwLock over Mutex:
/// - Multiple threads can read chromosomes simultaneously (read lock)
/// - Only one thread can write at a time (write lock)
/// - Better performance for read-heavy workloads
/// - No blocking of readers when other readers are active
pub struct FileChromosomeRepository {
    file_path: PathBuf,
    // RwLock allows multiple concurrent readers or one exclusive writer
    file_lock: RwLock<()>,
}

impl FileChromosomeRepository {
    pub fn new<P: AsRef<Path>>(file_path: P) -> Result<Self, String> {
        let original_path = file_path.as_ref().to_path_buf();
        
        // Create the chromosomes folder in the same directory as the original file
        let chromosomes_dir = original_path.parent()
            .unwrap_or(Path::new("."))
            .join("chromosomes");
        
        // Create the chromosomes directory if it doesn't exist
        fs::create_dir_all(&chromosomes_dir)
            .map_err(|e| format!("Failed to create chromosomes directory: {e}"))?;
        
        // Place the file inside the chromosomes folder
        let file_name = original_path.file_name()
            .unwrap_or(std::ffi::OsStr::new("chromosomes.json"));
        let path = chromosomes_dir.join(file_name);
        
        // Create empty file if it doesn't exist
        if !path.exists() {
            Self::write_tournament_history_to_file(&path, &TournamentHistory::new())?;
        }
        
        Ok(Self {
            file_path: path,
            file_lock: RwLock::new(()),
        })
    }
    
    fn read_chromosomes_from_file(file_path: &Path) -> Result<Vec<Chromosome>, String> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file: {e}"))?;
        
        if content.trim().is_empty() {
            return Ok(Vec::new());
        }
        
        let history: TournamentHistory = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse tournament history JSON: {e}"))?;
        
        Ok(history.get_latest_winners())
    }
    
    fn read_tournament_history_from_file(file_path: &Path) -> Result<TournamentHistory, String> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file: {e}"))?;
        
        if content.trim().is_empty() {
            return Ok(TournamentHistory::new());
        }
        
        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse tournament history JSON: {e}"))
    }
    
    fn write_tournament_history_to_file(file_path: &Path, history: &TournamentHistory) -> Result<(), String> {
        // Use atomic write operation: write to temporary file first, then rename
        let temp_path = file_path.with_extension("tmp");
        
        // Write to temporary file
        {
            let json = serde_json::to_string_pretty(history)
                .map_err(|e| format!("Failed to serialize tournament history: {e}"))?;
            
            let mut temp_file = fs::File::create(&temp_path)
                .map_err(|e| format!("Failed to create temporary file: {e}"))?;
            
            temp_file.write_all(json.as_bytes())
                .map_err(|e| format!("Failed to write to temporary file: {e}"))?;
            
            temp_file.sync_all()
                .map_err(|e| format!("Failed to sync temporary file: {e}"))?;
        }
        
        // Atomically replace the original file
        fs::rename(&temp_path, file_path)
            .map_err(|e| format!("Failed to replace file: {e}"))?;
        
        Ok(())
    }
}

impl ChromosomeRepository for FileChromosomeRepository {
    fn read_chromosomes(&self) -> Result<Vec<Chromosome>, String> {
        // Use read lock - allows multiple concurrent readers
        let _read_lock = self.file_lock.read()
            .map_err(|_| "Failed to acquire read lock for reading chromosomes")?;
        
        Self::read_chromosomes_from_file(&self.file_path)
    }
    
    fn write_chromosomes(&mut self, chromosomes: &[Chromosome]) -> Result<(), String> {
        // Use write lock - exclusive access for writing
        let _write_lock = self.file_lock.write()
            .map_err(|_| "Failed to acquire write lock for writing chromosomes")?;
        
        // Read existing history first
        let mut history = Self::read_tournament_history_from_file(&self.file_path)?;
        
        // Add new tournament with these chromosomes
        if !chromosomes.is_empty() {
            history.add_tournament(chromosomes.to_vec(), chromosomes.len() as i32);
        }
        
        // Write updated history back
        Self::write_tournament_history_to_file(&self.file_path, &history)
    }
    
    fn write_tournament_winners(&mut self, winners: &[Chromosome]) -> Result<(), String> {
        self.write_tournament_winners_with_count(winners, winners.len() as i32)
    }
    
    fn write_tournament_winners_with_count(&mut self, winners: &[Chromosome], player_count: i32) -> Result<(), String> {
        // Use write lock - exclusive access for writing
        let _write_lock = self.file_lock.write()
            .map_err(|_| "Failed to acquire write lock for writing tournament winners")?;
        
        // Read existing history first
        let mut history = Self::read_tournament_history_from_file(&self.file_path)?;
        
        // Add new tournament with winners
        if !winners.is_empty() {
            history.add_tournament(winners.to_vec(), player_count);
        }
        
        // Write updated history back
        Self::write_tournament_history_to_file(&self.file_path, &history)
    }
    
    fn validate_player_count(&self, player_count: i32) -> Result<(), String> {
        let _read_lock = self.file_lock.read()
            .map_err(|_| "Failed to acquire read lock for validation")?;
        
        let history = Self::read_tournament_history_from_file(&self.file_path)?;
        history.validate_player_count(player_count)
    }
}