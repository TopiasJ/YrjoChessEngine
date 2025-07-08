use chess::{Board, ChessMove, Color, Piece};
use rand::{Rng, SeedableRng, rngs::StdRng};

/// Zobrist hash keys for position hashing
#[derive(Debug)]
pub struct ZobristKeys {
    /// Hash keys for pieces on squares [piece][square]
    piece_square: [[u64; 64]; 12],
    /// Hash key for side to move (black)
    side_to_move: u64,
    /// Hash keys for castling rights [white_king][white_queen][black_king][black_queen]
    castling: [u64; 16],
    /// Hash keys for en passant file [file] (0-7)
    en_passant: [u64; 8],
}

impl ZobristKeys {
    /// Create new Zobrist keys with deterministic seed for consistency
    pub fn new() -> Self {
        let mut rng = StdRng::seed_from_u64(12345); // Fixed seed for reproducibility
        
        let mut piece_square = [[0u64; 64]; 12];
        for piece_idx in 0..12 {
            for square_idx in 0..64 {
                piece_square[piece_idx][square_idx] = rng.random();
            }
        }
        
        let side_to_move = rng.random();
        
        let mut castling = [0u64; 16];
        for i in 0..16 {
            castling[i] = rng.random();
        }
        
        let mut en_passant = [0u64; 8];
        for i in 0..8 {
            en_passant[i] = rng.random();
        }
        
        Self {
            piece_square,
            side_to_move,
            castling,
            en_passant,
        }
    }
    
    /// Get piece index for Zobrist hashing (0-11)
    fn piece_index(piece: Piece, color: Color) -> usize {
        let piece_idx = match piece {
            Piece::Pawn => 0,
            Piece::Knight => 1,
            Piece::Bishop => 2,
            Piece::Rook => 3,
            Piece::Queen => 4,
            Piece::King => 5,
        };
        
        match color {
            Color::White => piece_idx,
            Color::Black => piece_idx + 6,
        }
    }
    
    /// Compute Zobrist hash for a position (optimized version)
    #[inline]
    pub fn hash_position(&self, board: &Board) -> u64 {
        let mut hash = 0u64;
        
        // Hash pieces using bitboards for better performance
        for color in [Color::White, Color::Black] {
            let color_pieces = board.color_combined(color);
            for piece_type in [
                chess::Piece::Pawn,
                chess::Piece::Knight,
                chess::Piece::Bishop,
                chess::Piece::Rook,
                chess::Piece::Queen,
                chess::Piece::King,
            ] {
                let piece_bitboard = board.pieces(piece_type) & color_pieces;
                let piece_idx = Self::piece_index(piece_type, color);
                
                // Iterate through set bits efficiently
                for square in piece_bitboard {
                    hash ^= self.piece_square[piece_idx][square.to_index()];
                }
            }
        }
        
        // Hash side to move (only if black)
        if board.side_to_move() == Color::Black {
            hash ^= self.side_to_move;
        }
        
        // Hash castling rights (optimized)
        let white_rights = board.castle_rights(Color::White);
        let black_rights = board.castle_rights(Color::Black);
        let castling_idx = (white_rights.has_kingside() as usize)
            | ((white_rights.has_queenside() as usize) << 1)
            | ((black_rights.has_kingside() as usize) << 2)
            | ((black_rights.has_queenside() as usize) << 3);
        hash ^= self.castling[castling_idx];
        
        // Hash en passant
        if let Some(en_passant_square) = board.en_passant() {
            hash ^= self.en_passant[en_passant_square.get_file().to_index()];
        }
        
        hash
    }
}

/// Node type for transposition table entries
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeType {
    /// Exact value (PV node)
    Exact,
    /// Lower bound (Cut node - beta cutoff)
    LowerBound,
    /// Upper bound (All node - all moves failed low)
    UpperBound,
}

/// Transposition table entry
#[derive(Debug, Clone)]
pub struct TTEntry {
    /// Zobrist key for verification
    pub key: u64,
    /// Search depth when stored
    pub depth: i32,
    /// Node type
    pub node_type: NodeType,
    /// Evaluation score
    pub score: i32,
    /// Best move found (if any)
    pub best_move: Option<ChessMove>,
    /// Age/generation for replacement policy
    pub age: u8,
}

impl TTEntry {
    pub fn new(key: u64, depth: i32, node_type: NodeType, score: i32, best_move: Option<ChessMove>, age: u8) -> Self {
        Self {
            key,
            depth,
            node_type,
            score,
            best_move,
            age,
        }
    }
}

/// Transposition Table for storing position evaluations
pub struct TranspositionTable {
    /// The hash table storing entries - using Vec for better performance
    table: Vec<Option<TTEntry>>,
    /// Zobrist keys for hashing
    zobrist: ZobristKeys,
    /// Current age/generation
    current_age: u8,
    /// Size mask for modulo operation (table size must be power of 2)
    size_mask: usize,
    /// Statistics (only in debug mode to reduce overhead)
    #[cfg(debug_assertions)]
    pub hits: u64,
    #[cfg(debug_assertions)]
    pub misses: u64,
    #[cfg(debug_assertions)]
    pub collisions: u64,
}

impl TranspositionTable {
    /// Create new transposition table with specified size (must be power of 2)
    pub fn new(size: usize) -> Self {
        // Ensure size is power of 2
        let size = if size.is_power_of_two() { size } else { size.next_power_of_two() };
        
        Self {
            table: vec![None; size],
            zobrist: ZobristKeys::new(),
            current_age: 0,
            size_mask: size - 1,
            #[cfg(debug_assertions)]
            hits: 0,
            #[cfg(debug_assertions)]
            misses: 0,
            #[cfg(debug_assertions)]
            collisions: 0,
        }
    }
    
    /// Create default transposition table (1M entries)
    pub fn default() -> Self {
        Self::new(1_048_576) // 2^20 = 1M entries
    }
    
    /// Get Zobrist hash for a position
    pub fn hash_position(&self, board: &Board) -> u64 {
        self.zobrist.hash_position(board)
    }
    
    /// Probe the transposition table
    #[inline]
    pub fn probe(&mut self, key: u64) -> Option<&TTEntry> {
        let index = (key as usize) & self.size_mask;
        if let Some(entry) = &self.table[index] {
            if entry.key == key {
                #[cfg(debug_assertions)]
                { self.hits += 1; }
                Some(entry)
            } else {
                #[cfg(debug_assertions)]
                { self.collisions += 1; }
                None
            }
        } else {
            #[cfg(debug_assertions)]
            { self.misses += 1; }
            None
        }
    }
    
    /// Store entry in transposition table
    #[inline]
    pub fn store(&mut self, key: u64, depth: i32, node_type: NodeType, score: i32, best_move: Option<ChessMove>) {
        let index = (key as usize) & self.size_mask;
        let entry = TTEntry::new(key, depth, node_type, score, best_move, self.current_age);
        
        // Simple replacement strategy: replace if deeper or same depth but newer age
        let should_replace = if let Some(existing) = &self.table[index] {
            existing.key != key || // Hash collision
            depth >= existing.depth || // Deeper search
            (depth == existing.depth && self.current_age > existing.age) // Same depth but newer
        } else {
            true // Empty slot
        };
        
        if should_replace {
            self.table[index] = Some(entry);
        }
    }
    
    /// Advance age (call at start of new search)
    pub fn new_search(&mut self) {
        self.current_age = self.current_age.wrapping_add(1);
    }
    
    /// Clear the transposition table
    pub fn clear(&mut self) {
        for slot in &mut self.table {
            *slot = None;
        }
        #[cfg(debug_assertions)]
        {
            self.hits = 0;
            self.misses = 0;
            self.collisions = 0;
        }
    }
    
    /// Get hit rate as percentage
    pub fn hit_rate(&self) -> f64 {
        #[cfg(debug_assertions)]
        {
            let total = self.hits + self.misses;
            if total > 0 {
                (self.hits as f64 / total as f64) * 100.0
            } else {
                0.0
            }
        }
        #[cfg(not(debug_assertions))]
        {
            0.0 // No stats in release mode
        }
    }
    
    /// Get table statistics
    pub fn stats(&self) -> (usize, f64, u64, u64, u64) {
        #[cfg(debug_assertions)]
        {
            (
                self.table.len(),
                self.hit_rate(),
                self.hits,
                self.misses,
                self.collisions,
            )
        }
        #[cfg(not(debug_assertions))]
        {
            (self.table.len(), 0.0, 0, 0, 0) // No stats in release mode
        }
    }
}