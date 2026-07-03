use chess::ChessMove;

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
            current_age: 0,
            size_mask: size - 1,
            hits: 0,
            misses: 0,
            collisions: 0,
        }
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
