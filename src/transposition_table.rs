use chess::{Board, ChessMove, Color, Piece};
use rand::{rngs::StdRng, RngExt, SeedableRng};

const ZOBRIST_SEED: u64 = 12345;
const PIECES: [Piece; 6] = [Piece::Pawn, Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen, Piece::King];

#[derive(Debug)]
struct ZobristKeys {
    piece_square: [[u64; 64]; 12],
    side_to_move: u64,
    castling: [u64; 16],
    en_passant: [u64; 8],
}

impl Default for ZobristKeys {
    fn default() -> Self {
        let mut rng = StdRng::seed_from_u64(ZOBRIST_SEED);
        Self {
            piece_square: std::array::from_fn(|_| std::array::from_fn(|_| rng.random())),
            side_to_move: rng.random(),
            castling: std::array::from_fn(|_| rng.random()),
            en_passant: std::array::from_fn(|_| rng.random()),
        }
    }
}

impl ZobristKeys {
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

    #[inline]
    fn hash_position(&self, board: &Board) -> u64 {
        let mut hash = 0u64;

        for color in [Color::White, Color::Black] {
            let color_pieces = board.color_combined(color);
            for piece in PIECES {
                let piece_bb = board.pieces(piece) & color_pieces;
                let idx = Self::piece_index(piece, color);
                for square in piece_bb {
                    hash ^= self.piece_square[idx][square.to_index()];
                }
            }
        }

        if board.side_to_move() == Color::Black {
            hash ^= self.side_to_move;
        }

        let white = board.castle_rights(Color::White);
        let black = board.castle_rights(Color::Black);
        let castling_idx = (white.has_kingside() as usize) | ((white.has_queenside() as usize) << 1) | ((black.has_kingside() as usize) << 2) | ((black.has_queenside() as usize) << 3);
        hash ^= self.castling[castling_idx];

        if let Some(ep) = board.en_passant() {
            hash ^= self.en_passant[ep.get_file().to_index()];
        }

        hash
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeType {
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Debug, Clone)]
pub struct TTEntry {
    pub key: u64,
    pub depth: i32,
    pub node_type: NodeType,
    pub score: i32,
    pub best_move: Option<ChessMove>,
    pub age: u8,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TtStats {
    pub hits: u64,
    pub misses: u64,
    pub collisions: u64,
}

/// Fixed-size hash table caching position evaluations across the search.
pub struct TranspositionTable {
    table: Vec<Option<TTEntry>>,
    zobrist: ZobristKeys,
    current_age: u8,
    size_mask: usize,
    hits: u64,
    misses: u64,
    collisions: u64,
}

impl Default for TranspositionTable {
    fn default() -> Self {
        Self::new(1 << 20) // 1M entries
    }
}

impl TranspositionTable {
    pub fn new(size: usize) -> Self {
        let size = size.next_power_of_two();
        Self {
            table: vec![None; size],
            zobrist: ZobristKeys::default(),
            current_age: 0,
            size_mask: size - 1,
            hits: 0,
            misses: 0,
            collisions: 0,
        }
    }

    pub fn hash_position(&self, board: &Board) -> u64 {
        self.zobrist.hash_position(board)
    }

    #[inline]
    pub fn probe(&mut self, key: u64) -> Option<&TTEntry> {
        let index = (key as usize) & self.size_mask;
        match &self.table[index] {
            Some(entry) if entry.key == key => {
                self.hits += 1;
                Some(entry)
            }
            Some(_) => {
                self.collisions += 1;
                None
            }
            None => {
                self.misses += 1;
                None
            }
        }
    }

    /// Store an entry, preferring deeper results and evicting entries from older searches.
    #[inline]
    pub fn store(&mut self, key: u64, depth: i32, node_type: NodeType, score: i32, best_move: Option<ChessMove>) {
        let index = (key as usize) & self.size_mask;
        let should_replace = match &self.table[index] {
            None => true,
            Some(existing) if existing.key == key => depth >= existing.depth,
            Some(existing) => existing.age != self.current_age || depth > existing.depth,
        };
        if should_replace {
            self.table[index] = Some(TTEntry {
                key,
                depth,
                node_type,
                score,
                best_move,
                age: self.current_age,
            });
        }
    }

    /// Begin a new search: bump generation and reset per-search counters.
    pub fn new_search(&mut self) {
        self.current_age = self.current_age.wrapping_add(1);
        self.hits = 0;
        self.misses = 0;
        self.collisions = 0;
    }

    pub fn stats(&self) -> TtStats {
        TtStats {
            hits: self.hits,
            misses: self.misses,
            collisions: self.collisions,
        }
    }
}
