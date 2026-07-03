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

/// Scores beyond this magnitude are mate scores (see `check_terminal_position`).
const MATE_SCORE_THRESHOLD: i32 = 9000;

/// Mate scores encode distance to mate via the storing node's remaining depth,
/// so they are only valid for the search that produced them. Re-base them to be
/// node-relative before storing, so an entry stays correct when the same
/// position is probed at a different remaining depth (e.g. by a later search
/// in the same game).
fn to_tt_score(score: i32, depth: i32) -> i32 {
    if score > MATE_SCORE_THRESHOLD {
        let rebased = score - depth;
        debug_assert!(rebased > MATE_SCORE_THRESHOLD, "search depth too large: re-based mate score no longer recognizable as a mate");
        rebased
    } else if score < -MATE_SCORE_THRESHOLD {
        let rebased = score + depth;
        debug_assert!(rebased < -MATE_SCORE_THRESHOLD, "search depth too large: re-based mate score no longer recognizable as a mate");
        rebased
    } else {
        score
    }
}

/// Inverse of `to_tt_score`: re-base a stored node-relative mate score to the
/// probing node's remaining depth.
fn from_tt_score(score: i32, depth: i32) -> i32 {
    if score > MATE_SCORE_THRESHOLD {
        score + depth
    } else if score < -MATE_SCORE_THRESHOLD {
        score - depth
    } else {
        score
    }
}

impl AlgorithmTraits for AlphaBetaAlgorithm {
    fn get_best_move(&mut self, board: Board, depth: i32) -> Option<ChessMove> {
        let best_moves = self.search_root_moves(board, depth, None);

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
        let best_moves = self.search_root_moves(board, depth, Some(chromosome));

        let selected_index = get_random_from_multiple_best_moves(&best_moves, board.side_to_move())?;
        let selected_move = match board.side_to_move() {
            Color::White => best_moves[best_moves.len() - 1 - selected_index],
            Color::Black => best_moves[selected_index],
        };

        Some(selected_move.0)
    }

    fn get_best_move_with_stats(&mut self, board: Board, depth: i32) -> (Option<ChessMove>, SearchStats) {
        let best_moves = self.search_root_moves(board, depth, None);

        let selected_index = get_random_from_multiple_best_moves(&best_moves, board.side_to_move());
        let selected_move = selected_index.map(|idx| match board.side_to_move() {
            Color::White => best_moves[best_moves.len() - 1 - idx].0,
            Color::Black => best_moves[idx].0,
        });

        self.update_stats_from_tt();
        (selected_move, self.stats.clone())
    }

    fn get_best_move_with_chromosome_and_stats(&mut self, board: Board, depth: i32, chromosome: &Chromosome) -> (Option<ChessMove>, SearchStats) {
        let best_moves = self.search_root_moves(board, depth, Some(chromosome));

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
    /// Search every root move and return them sorted by score (ascending).
    /// The window is narrowed as the best score improves, so ties with the
    /// best stay exact while strictly worse moves are pruned early.
    fn search_root_moves(&mut self, board: Board, depth: i32, chromosome: Option<&Chromosome>) -> Vec<(ChessMove, i32)> {
        self.reset_stats();
        let mut best_moves: Vec<(ChessMove, i32)> = Vec::new();
        // Captures first, like every other ply: a strong early best score
        // makes the narrowed window prune more of the remaining moves.
        let moves = self.get_ordered_moves(&board);

        let mut best_score: Option<i32> = None;
        for mov in moves {
            let score = self.calc_one_move(&mut best_moves, mov, board, depth, chromosome, best_score);
            best_score = Some(update_root_best(best_score, score, board.side_to_move()));
        }

        // Sort moves by evaluation (best for current player first)
        best_moves.sort_by_key(|k| k.1);
        best_moves
    }

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

        // Generate legal moves once for both the terminal check and move ordering.
        let movegen = MoveGen::new_legal(&board);
        if movegen.len() == 0 {
            self.stats.terminal_nodes += 1;
            return terminal_score(&board, depth_left_before);
        }

        if depth_left_before == 0 {
            self.stats.evaluations += 1;
            return match chromosome {
                Some(chr) => Evaluator::evaluate_with_chromosome(board, chr),
                None => Evaluator::evaluate(board),
            };
        }

        let mut alpha = alpha_before;
        let mut moves = ordered_moves(&board, movegen);
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

        // Generate legal moves once for both the terminal check and move ordering.
        let movegen = MoveGen::new_legal(&board);
        if movegen.len() == 0 {
            self.stats.terminal_nodes += 1;
            return terminal_score(&board, depth_left_before);
        }

        if depth_left_before == 0 {
            self.stats.evaluations += 1;
            return match chromosome {
                Some(chr) => Evaluator::evaluate_with_chromosome(board, chr),
                None => Evaluator::evaluate(board),
            };
        }

        let mut beta = beta_before;
        let mut moves = ordered_moves(&board, movegen);
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
        // The chess crate maintains this Zobrist hash incrementally on every
        // make_move_new, so reading it is O(1).
        let hash = board.get_hash();
        let (entry_depth, node_type, score, best_move) = match self.transposition_table.probe(hash) {
            Some(entry) => (entry.depth, entry.node_type, from_tt_score(entry.score, depth), entry.best_move),
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
        self.transposition_table.store(hash, depth, node_type, to_tt_score(score, depth), best_move);
    }

    /// Get moves in order of priority (captures first, then others)
    pub fn get_ordered_moves(&self, board: &Board) -> Vec<ChessMove> {
        ordered_moves(board, MoveGen::new_legal(board))
    }
}

/// Score for a position with no legal moves: stalemate is a draw, checkmate is
/// scored so the side to move loses, preferring faster mates.
fn terminal_score(board: &Board, depth_left: i32) -> i32 {
    if board.checkers() == &EMPTY {
        0 // Stalemate
    } else {
        match board.side_to_move() {
            Color::White => -9999 - depth_left,
            Color::Black => 9999 + depth_left,
        }
    }
}

/// Build the ordered move list (captures first, then others), reusing an
/// already-created generator for the capture pass instead of regenerating.
fn ordered_moves(board: &Board, mut capture_moves: MoveGen) -> Vec<ChessMove> {
    let mut moves: Vec<ChessMove> = Vec::with_capacity(capture_moves.len());

    // First, collect capture moves
    let targets = board.color_combined(!board.side_to_move());
    capture_moves.set_iterator_mask(*targets);
    moves.extend(capture_moves);

    // Then, collect non-capture moves
    let mut non_capture_moves = MoveGen::new_legal(board);
    non_capture_moves.set_iterator_mask(!*targets);
    moves.extend(non_capture_moves);

    moves
}

#[cfg(test)]
mod root_search_tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn update_root_best_tracks_side_preference() {
        assert_eq!(update_root_best(None, 5, Color::White), 5);
        assert_eq!(update_root_best(Some(3), 5, Color::White), 5);
        assert_eq!(update_root_best(Some(7), 5, Color::White), 7);
        assert_eq!(update_root_best(None, 5, Color::Black), 5);
        assert_eq!(update_root_best(Some(3), 5, Color::Black), 3);
        assert_eq!(update_root_best(Some(7), 5, Color::Black), 5);
    }

    /// The invariant the random tie-break at the root depends on: every move
    /// that ties the best score must be reported with its exact full-window
    /// value, and no pruned move may collide with that best score.
    #[test]
    fn narrowed_root_windows_keep_best_scores_exact() {
        for fen in [
            "r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 3",
            "r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 4 3",
        ] {
            let board = Board::from_str(fen).unwrap();
            let depth = 3;

            let mut alg = AlphaBetaAlgorithm::new();
            let narrowed = alg.search_root_moves(board, depth, None);

            // Reference scores: each move searched with the full window by a
            // fresh instance, so nothing is pruned.
            let exact: Vec<(ChessMove, i32)> = narrowed
                .iter()
                .map(|&(mov, _)| {
                    let mut reference = AlphaBetaAlgorithm::new();
                    let new_board = board.make_move_new(mov);
                    let score = match new_board.side_to_move() {
                        Color::White => reference.alpha_beta_max(new_board, -999999, 999999, depth, None),
                        Color::Black => reference.alpha_beta_min(new_board, -999999, 999999, depth, None),
                    };
                    (mov, score)
                })
                .collect();

            let best = match board.side_to_move() {
                Color::White => exact.iter().map(|k| k.1).max().unwrap(),
                Color::Black => exact.iter().map(|k| k.1).min().unwrap(),
            };

            for (&(mov, narrowed_score), &(_, exact_score)) in narrowed.iter().zip(exact.iter()) {
                if exact_score == best {
                    assert_eq!(narrowed_score, exact_score, "tied-best move {mov} must keep its exact score");
                } else {
                    assert_ne!(narrowed_score, best, "pruned move {mov} must not collide with the best score");
                }
            }
        }
    }
}

#[cfg(test)]
mod tt_score_tests {
    use super::*;

    /// A mate 2 plies below a node searched with 5 plies remaining
    /// (raw score 9999 + 3) must read back as the same mate distance when the
    /// position is probed by a search with 7 plies remaining.
    #[test]
    fn mate_scores_round_trip_across_depths() {
        let stored = to_tt_score(9999 + 3, 5);
        assert_eq!(from_tt_score(stored, 7), 9999 + 5);

        let stored = to_tt_score(-9999 - 3, 5);
        assert_eq!(from_tt_score(stored, 7), -9999 - 5);
    }

    #[test]
    fn non_mate_scores_are_untouched() {
        assert_eq!(to_tt_score(150, 7), 150);
        assert_eq!(to_tt_score(-150, 7), -150);
        assert_eq!(from_tt_score(150, 7), 150);
        assert_eq!(from_tt_score(-150, 7), -150);
    }
}
