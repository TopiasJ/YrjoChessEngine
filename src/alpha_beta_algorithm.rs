use crate::chromosome::Chromosome;
use crate::evaluator::Evaluator;
use crate::transposition_table::{NodeType, TranspositionTable};
use chess::{Board, ChessMove, Color, MoveGen, EMPTY};
use rand::RngExt;

#[derive(Debug, Clone)]
pub struct SearchStats {
    pub nodes_searched: u64,
    pub evaluations: u64,
    pub cutoffs: u64,
    pub terminal_nodes: u64,
    pub tt_hits: u64,
    pub tt_misses: u64,
    pub tt_collisions: u64,
}

impl Default for SearchStats {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchStats {
    pub fn new() -> Self {
        Self {
            nodes_searched: 0,
            evaluations: 0,
            cutoffs: 0,
            terminal_nodes: 0,
            tt_hits: 0,
            tt_misses: 0,
            tt_collisions: 0,
        }
    }
}

pub trait AlgorithmTraits {
    fn get_best_move(&mut self, board: Board, depth: i32) -> Option<ChessMove>;
    fn get_best_move_with_chromosome(&mut self, board: Board, depth: i32, chromosome: &Chromosome) -> Option<ChessMove>;
    fn get_best_move_with_stats(&mut self, board: Board, depth: i32) -> (Option<ChessMove>, SearchStats);
    fn get_best_move_with_chromosome_and_stats(&mut self, board: Board, depth: i32, chromosome: &Chromosome) -> (Option<ChessMove>, SearchStats);
}

pub struct AlphaBetaAlgorithm {
    pub stats: SearchStats,
    pub transposition_table: TranspositionTable,
}

impl Default for AlphaBetaAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}

impl AlphaBetaAlgorithm {
    pub fn new() -> Self {
        Self {
            stats: SearchStats::new(),
            transposition_table: TranspositionTable::default(),
        }
    }

    pub fn reset_stats(&mut self) {
        self.stats = SearchStats::new();
        self.transposition_table.new_search();
    }

    fn update_stats_from_tt(&mut self) {
        let tt = self.transposition_table.stats();
        self.stats.tt_hits = tt.hits;
        self.stats.tt_misses = tt.misses;
        self.stats.tt_collisions = tt.collisions;
    }
}

/// Outcome of a transposition-table probe at the start of a search node.
enum ProbeResult {
    /// The cached entry already determines the score; the caller should return it directly.
    Cutoff(i32),
    /// No usable cached score; continue searching with the given hash and (optional) hint move.
    Continue { hash: u64, tt_move: Option<ChessMove> },
}

/// Below this depth the TT lookup/store cost outweighs the savings, so we skip it.
const TT_MIN_DEPTH: i32 = 2;

impl AlgorithmTraits for AlphaBetaAlgorithm {
    fn get_best_move(&mut self, board: Board, depth: i32) -> Option<ChessMove> {
        self.reset_stats();
        let mut best_moves: Vec<(ChessMove, i32)> = Vec::new();
        let moves: Vec<ChessMove> = MoveGen::new_legal(&board).collect();

        // Evaluate all moves, narrowing the window as the best score improves
        let mut best_score: Option<i32> = None;
        for mov in moves {
            let score = self.calc_one_move(&mut best_moves, mov, board, depth, None, best_score);
            best_score = Some(update_root_best(best_score, score, board.side_to_move()));
        }

        // Sort moves by evaluation (best for current player first)
        best_moves.sort_by_key(|k| k.1);

        let selected_index = get_random_from_multiple_best_moves(&best_moves, board.side_to_move())?;
        let selected_move = match board.side_to_move() {
            Color::White => best_moves[best_moves.len() - 1 - selected_index],
            Color::Black => best_moves[selected_index],
        };

        let color: String = match board.side_to_move() {
            Color::White => "White".to_string(),
            Color::Black => "Black".to_string(),
        };
        println!("value for selected (normal), move for {0}: {1}", color, selected_move.1);
        Some(selected_move.0)
    }

    fn get_best_move_with_chromosome(&mut self, board: Board, depth: i32, chromosome: &Chromosome) -> Option<ChessMove> {
        self.reset_stats();
        let mut best_moves: Vec<(ChessMove, i32)> = Vec::new();
        let moves: Vec<ChessMove> = MoveGen::new_legal(&board).collect();

        // Evaluate all moves, narrowing the window as the best score improves
        let mut best_score: Option<i32> = None;
        for mov in moves {
            let score = self.calc_one_move(&mut best_moves, mov, board, depth, Some(chromosome), best_score);
            best_score = Some(update_root_best(best_score, score, board.side_to_move()));
        }

        // Sort moves by evaluation (best for current player first)
        best_moves.sort_by_key(|k| k.1);

        let selected_index = get_random_from_multiple_best_moves(&best_moves, board.side_to_move())?;
        let selected_move = match board.side_to_move() {
            Color::White => best_moves[best_moves.len() - 1 - selected_index],
            Color::Black => best_moves[selected_index],
        };

        //let color: String = match board.side_to_move() {
        //    Color::White => "White".to_string(),
        //    Color::Black => "Black".to_string(),
        //};
        // println!("value for selected move for {0}: {1}", color, selected_move.1);
        Some(selected_move.0)
    }

    fn get_best_move_with_stats(&mut self, board: Board, depth: i32) -> (Option<ChessMove>, SearchStats) {
        self.reset_stats();
        let mut best_moves: Vec<(ChessMove, i32)> = Vec::new();
        let moves: Vec<ChessMove> = MoveGen::new_legal(&board).collect();

        // Evaluate all moves, narrowing the window as the best score improves
        let mut best_score: Option<i32> = None;
        for mov in moves {
            let score = self.calc_one_move(&mut best_moves, mov, board, depth, None, best_score);
            best_score = Some(update_root_best(best_score, score, board.side_to_move()));
        }

        // Sort moves by evaluation (best for current player first)
        best_moves.sort_by_key(|k| k.1);

        let selected_index = get_random_from_multiple_best_moves(&best_moves, board.side_to_move());
        let selected_move = selected_index.map(|idx| match board.side_to_move() {
            Color::White => best_moves[best_moves.len() - 1 - idx].0,
            Color::Black => best_moves[idx].0,
        });

        self.update_stats_from_tt();
        (selected_move, self.stats.clone())
    }

    fn get_best_move_with_chromosome_and_stats(&mut self, board: Board, depth: i32, chromosome: &Chromosome) -> (Option<ChessMove>, SearchStats) {
        self.reset_stats();
        let mut best_moves: Vec<(ChessMove, i32)> = Vec::new();
        let moves: Vec<ChessMove> = MoveGen::new_legal(&board).collect();

        // Evaluate all moves, narrowing the window as the best score improves
        let mut best_score: Option<i32> = None;
        for mov in moves {
            let score = self.calc_one_move(&mut best_moves, mov, board, depth, Some(chromosome), best_score);
            best_score = Some(update_root_best(best_score, score, board.side_to_move()));
        }

        // Sort moves by evaluation (best for current player first)
        best_moves.sort_by_key(|k| k.1);

        let selected_index = get_random_from_multiple_best_moves(&best_moves, board.side_to_move());
        let selected_move = selected_index.map(|idx| match board.side_to_move() {
            Color::White => best_moves[best_moves.len() - 1 - idx].0,
            Color::Black => best_moves[idx].0,
        });

        self.update_stats_from_tt();
        (selected_move, self.stats.clone())
    }
}

/// Move the TT hint move to the front of the move list so it's searched first.
fn order_tt_move_first(moves: &mut [ChessMove], tt_move: Option<ChessMove>) {
    let Some(tt_mv) = tt_move else { return };
    if let Some(pos) = moves.iter().position(|&mv| mv == tt_mv) {
        moves.swap(0, pos);
    }
}

fn get_random_from_multiple_best_moves(best_moves: &Vec<(ChessMove, i32)>, color: Color) -> Option<usize> {
    let best_value = match color {
        Color::White => best_moves.last()?.1,
        Color::Black => best_moves.first()?.1,
    };
    let mut amount_of_equal_moves = 0;
    for moves in best_moves {
        if moves.1 == best_value {
            amount_of_equal_moves += 1;
        }
    }
    let selected_index: i32 = rand::rng().random_range(0..amount_of_equal_moves);
    Some(selected_index as usize)
}

/// Fold a root move's score into the running best for the side to move.
fn update_root_best(best_so_far: Option<i32>, score: i32, side: Color) -> i32 {
    match (best_so_far, side) {
        (None, _) => score,
        (Some(best), Color::White) => best.max(score),
        (Some(best), Color::Black) => best.min(score),
    }
}

impl AlphaBetaAlgorithm {
    #[inline]
    fn calc_one_move(&mut self, best_moves: &mut Vec<(ChessMove, i32)>, a_move: ChessMove, test_game: Board, depth: i32, chromosome: Option<&Chromosome>, best_so_far: Option<i32>) -> i32 {
        let new_board = test_game.make_move_new(a_move);
        // Narrow the window to one point past the best score so far: moves that tie
        // the best still get exact scores (needed for the random tie-break at the
        // root), while strictly worse moves fail early and get pruned.
        let result: i32 = match new_board.side_to_move() {
            Color::White => {
                let beta = best_so_far.map_or(999999, |best| best + 1);
                self.alpha_beta_max(new_board, -999999, beta, depth, chromosome)
            }
            Color::Black => {
                let alpha = best_so_far.map_or(-999999, |best| best - 1);
                self.alpha_beta_min(new_board, alpha, 999999, depth, chromosome)
            }
        };
        best_moves.push((a_move, result));
        result
    }

    pub fn alpha_beta_max(&mut self, board: Board, alpha_before: i32, beta: i32, depth_left_before: i32, chromosome: Option<&Chromosome>) -> i32 {
        self.stats.nodes_searched += 1;

        let (board_hash, tt_move) = match self.probe_tt(&board, depth_left_before, alpha_before, beta) {
            ProbeResult::Cutoff(score) => return score,
            ProbeResult::Continue { hash, tt_move } => (hash, tt_move),
        };

        if let Some(terminal_score) = self.check_terminal_position(&board, depth_left_before) {
            self.stats.terminal_nodes += 1;
            return terminal_score;
        }

        if depth_left_before == 0 {
            self.stats.evaluations += 1;
            return match chromosome {
                Some(chr) => Evaluator::evaluate_with_chromosome(board, chr),
                None => Evaluator::evaluate(board),
            };
        }

        let mut alpha = alpha_before;
        let mut moves = self.get_ordered_moves(&board);
        order_tt_move_first(&mut moves, tt_move);
        let mut best_move = None;

        for mov in moves {
            let new_board = board.make_move_new(mov);
            let score = self.alpha_beta_min(new_board, alpha, beta, depth_left_before - 1, chromosome);

            if score >= beta {
                self.stats.cutoffs += 1;
                self.store_tt(board_hash, depth_left_before, NodeType::LowerBound, beta, Some(mov));
                return beta;
            }
            if score > alpha {
                alpha = score;
                best_move = Some(mov);
            }
        }

        let node_type = if alpha <= alpha_before { NodeType::UpperBound } else { NodeType::Exact };
        self.store_tt(board_hash, depth_left_before, node_type, alpha, best_move);

        alpha
    }

    pub fn alpha_beta_min(&mut self, board: Board, alpha: i32, beta_before: i32, depth_left_before: i32, chromosome: Option<&Chromosome>) -> i32 {
        self.stats.nodes_searched += 1;

        let (board_hash, tt_move) = match self.probe_tt(&board, depth_left_before, alpha, beta_before) {
            ProbeResult::Cutoff(score) => return score,
            ProbeResult::Continue { hash, tt_move } => (hash, tt_move),
        };

        if let Some(terminal_score) = self.check_terminal_position(&board, depth_left_before) {
            self.stats.terminal_nodes += 1;
            return terminal_score;
        }

        if depth_left_before == 0 {
            self.stats.evaluations += 1;
            return match chromosome {
                Some(chr) => Evaluator::evaluate_with_chromosome(board, chr),
                None => Evaluator::evaluate(board),
            };
        }

        let mut beta = beta_before;
        let mut moves = self.get_ordered_moves(&board);
        order_tt_move_first(&mut moves, tt_move);
        let mut best_move = None;

        for mov in moves {
            let new_board = board.make_move_new(mov);
            let score = self.alpha_beta_max(new_board, alpha, beta, depth_left_before - 1, chromosome);

            if score <= alpha {
                self.stats.cutoffs += 1;
                self.store_tt(board_hash, depth_left_before, NodeType::UpperBound, alpha, Some(mov));
                return alpha;
            }
            if score < beta {
                beta = score;
                best_move = Some(mov);
            }
        }

        let node_type = if beta >= beta_before { NodeType::LowerBound } else { NodeType::Exact };
        self.store_tt(board_hash, depth_left_before, node_type, beta, best_move);

        beta
    }

    fn probe_tt(&mut self, board: &Board, depth: i32, alpha: i32, beta: i32) -> ProbeResult {
        if depth < TT_MIN_DEPTH {
            return ProbeResult::Continue { hash: 0, tt_move: None };
        }
        let hash = self.transposition_table.hash_position(board);
        let (entry_depth, node_type, score, best_move) = match self.transposition_table.probe(hash) {
            Some(entry) => (entry.depth, entry.node_type, entry.score, entry.best_move),
            None => return ProbeResult::Continue { hash, tt_move: None },
        };
        if entry_depth >= depth {
            match node_type {
                NodeType::Exact => return ProbeResult::Cutoff(score),
                NodeType::LowerBound if score >= beta => return ProbeResult::Cutoff(score),
                NodeType::UpperBound if score <= alpha => return ProbeResult::Cutoff(score),
                _ => {}
            }
        }
        ProbeResult::Continue { hash, tt_move: best_move }
    }

    fn store_tt(&mut self, hash: u64, depth: i32, node_type: NodeType, score: i32, best_move: Option<ChessMove>) {
        if depth < TT_MIN_DEPTH {
            return;
        }
        self.transposition_table.store(hash, depth, node_type, score, best_move);
    }

    /// Check if the position is terminal (game over) and return the appropriate score
    fn check_terminal_position(&self, board: &Board, depth_left: i32) -> Option<i32> {
        let moves_iterable = MoveGen::new_legal(board);

        if moves_iterable.len() == 0 {
            // Game ended - check if it's checkmate or stalemate
            if board.checkers() == &EMPTY {
                return Some(0); // Stalemate
            } else {
                // Checkmate - the side to move is checkmated
                return Some(match board.side_to_move() {
                    Color::White => -9999 - depth_left,
                    Color::Black => 9999 + depth_left,
                });
            }
        }

        None // Game continues
    }

    /// Get moves in order of priority (captures first, then others)
    pub fn get_ordered_moves(&self, board: &Board) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();

        // First, collect capture moves
        let mut capture_moves = MoveGen::new_legal(board);
        let targets = board.color_combined(!board.side_to_move());
        capture_moves.set_iterator_mask(*targets);
        moves.extend(capture_moves);

        // Then, collect non-capture moves
        let mut non_capture_moves = MoveGen::new_legal(board);
        non_capture_moves.set_iterator_mask(!*targets);
        moves.extend(non_capture_moves);

        moves
    }
}
