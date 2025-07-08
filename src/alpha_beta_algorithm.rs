use crate::chromosome::Chromosome;
use crate::evaluator::Evaluator;
use crate::transposition_table::{TranspositionTable, NodeType};
use chess::{Board, ChessMove, Color, MoveGen, EMPTY};
use rand::Rng;

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

impl AlphaBetaAlgorithm {
    pub fn new() -> Self {
        Self {
            stats: SearchStats::new(),
            transposition_table: TranspositionTable::default(),
        }
    }
    
    pub fn reset_stats(&mut self) {
        self.stats = SearchStats::new();
        // Start new search in transposition table
        self.transposition_table.new_search();
    }
    
    pub fn update_stats_from_tt(&mut self) {
        // Update TT stats from transposition table
        let (_, _, hits, misses, collisions) = self.transposition_table.stats();
        self.stats.tt_hits = hits;
        self.stats.tt_misses = misses;
        self.stats.tt_collisions = collisions;
    }
}

impl AlgorithmTraits for AlphaBetaAlgorithm {
    fn get_best_move(&mut self, board: Board, depth: i32) -> Option<ChessMove> {
        self.reset_stats();
        let mut best_moves: Vec<(ChessMove, i32)> = Vec::new();
        let moves: Vec<ChessMove> = MoveGen::new_legal(&board).collect();

        // Evaluate all moves
        for mov in moves {
            self.calc_one_move(&mut best_moves, mov, board, depth, None);
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

        // Evaluate all moves
        for mov in moves {
            self.calc_one_move(&mut best_moves, mov, board, depth, Some(chromosome));
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

        // Evaluate all moves
        for mov in moves {
            self.calc_one_move(&mut best_moves, mov, board, depth, None);
        }

        // Sort moves by evaluation (best for current player first)
        best_moves.sort_by_key(|k| k.1);

        let selected_index = get_random_from_multiple_best_moves(&best_moves, board.side_to_move());
        let selected_move = if let Some(idx) = selected_index {
            Some(match board.side_to_move() {
                Color::White => best_moves[best_moves.len() - 1 - idx].0,
                Color::Black => best_moves[idx].0,
            })
        } else {
            None
        };

        self.update_stats_from_tt();
        (selected_move, self.stats.clone())
    }
    
    fn get_best_move_with_chromosome_and_stats(&mut self, board: Board, depth: i32, chromosome: &Chromosome) -> (Option<ChessMove>, SearchStats) {
        self.reset_stats();
        let mut best_moves: Vec<(ChessMove, i32)> = Vec::new();
        let moves: Vec<ChessMove> = MoveGen::new_legal(&board).collect();

        // Evaluate all moves
        for mov in moves {
            self.calc_one_move(&mut best_moves, mov, board, depth, Some(chromosome));
        }

        // Sort moves by evaluation (best for current player first)
        best_moves.sort_by_key(|k| k.1);

        let selected_index = get_random_from_multiple_best_moves(&best_moves, board.side_to_move());
        let selected_move = if let Some(idx) = selected_index {
            Some(match board.side_to_move() {
                Color::White => best_moves[best_moves.len() - 1 - idx].0,
                Color::Black => best_moves[idx].0,
            })
        } else {
            None
        };

        self.update_stats_from_tt();
        (selected_move, self.stats.clone())
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

impl AlphaBetaAlgorithm {
    #[inline]
    fn calc_one_move(&mut self, best_moves: &mut Vec<(ChessMove, i32)>, a_move: ChessMove, test_game: Board, depth: i32, chromosome: Option<&Chromosome>) {
        let new_board = test_game.make_move_new(a_move);
        let result: i32 = match new_board.side_to_move() {
            Color::White => self.alpha_beta_max(new_board, -999999, 999999, depth, chromosome),
            Color::Black => self.alpha_beta_min(new_board, -999999, 999999, depth, chromosome),
        };
        best_moves.push((a_move, result));
    }

    pub fn alpha_beta_max(&mut self, board: Board, alpha_before: i32, beta: i32, depth_left_before: i32, chromosome: Option<&Chromosome>) -> i32 {
        self.stats.nodes_searched += 1;
        
        // Only use TT for deeper searches (depth >= 2) to avoid overhead
        let mut tt_move = None;
        let board_hash = if depth_left_before >= 2 {
            let hash = self.transposition_table.hash_position(&board);
            if let Some(tt_entry) = self.transposition_table.probe(hash) {
                if tt_entry.depth >= depth_left_before {
                    match tt_entry.node_type {
                        NodeType::Exact => return tt_entry.score,
                        NodeType::LowerBound => {
                            if tt_entry.score >= beta {
                                return tt_entry.score;
                            }
                        }
                        NodeType::UpperBound => {
                            if tt_entry.score <= alpha_before {
                                return tt_entry.score;
                            }
                        }
                    }
                }
                tt_move = tt_entry.best_move;
            }
            hash
        } else {
            0 // Don't compute hash for shallow searches
        };
        
        // Check for game end conditions
        if let Some(terminal_score) = self.check_terminal_position(&board, depth_left_before) {
            self.stats.terminal_nodes += 1;
            return terminal_score;
        }

        // Leaf node evaluation
        if depth_left_before == 0 {
            self.stats.evaluations += 1;
            return match chromosome {
                Some(chr) => Evaluator::evaluate_with_chromosome(board, chr),
                None => Evaluator::evaluate(board),
            };
        }

        let mut alpha = alpha_before;
        let original_alpha = alpha_before;
        let mut moves = self.get_ordered_moves(&board);
        
        // Order TT move first if available
        if let Some(tt_mv) = tt_move {
            if let Some(pos) = moves.iter().position(|&mv| mv == tt_mv) {
                moves.swap(0, pos);
            }
        }
        
        let mut best_move = None;

        for mov in moves {
            let new_board = board.make_move_new(mov);
            let score = self.alpha_beta_min(new_board, alpha, beta, depth_left_before - 1, chromosome);

            if score >= beta {
                self.stats.cutoffs += 1;
                // Store beta cutoff in TT (only for deeper searches)
                if depth_left_before >= 2 {
                    self.transposition_table.store(board_hash, depth_left_before, NodeType::LowerBound, beta, Some(mov));
                }
                return beta; // Beta cutoff
            }
            if score > alpha {
                alpha = score;
                best_move = Some(mov);
            }
        }
        
        // Store result in transposition table (only for deeper searches)
        if depth_left_before >= 2 {
            let node_type = if alpha <= original_alpha {
                NodeType::UpperBound // All moves failed low
            } else {
                NodeType::Exact // PV node
            };
            self.transposition_table.store(board_hash, depth_left_before, node_type, alpha, best_move);
        }

        alpha
    }

    pub fn alpha_beta_min(&mut self, board: Board, alpha: i32, beta_before: i32, depth_left_before: i32, chromosome: Option<&Chromosome>) -> i32 {
        self.stats.nodes_searched += 1;
        
        // Only use TT for deeper searches (depth >= 2) to avoid overhead
        let mut tt_move = None;
        let board_hash = if depth_left_before >= 2 {
            let hash = self.transposition_table.hash_position(&board);
            if let Some(tt_entry) = self.transposition_table.probe(hash) {
                if tt_entry.depth >= depth_left_before {
                    match tt_entry.node_type {
                        NodeType::Exact => return tt_entry.score,
                        NodeType::LowerBound => {
                            if tt_entry.score >= beta_before {
                                return tt_entry.score;
                            }
                        }
                        NodeType::UpperBound => {
                            if tt_entry.score <= alpha {
                                return tt_entry.score;
                            }
                        }
                    }
                }
                tt_move = tt_entry.best_move;
            }
            hash
        } else {
            0 // Don't compute hash for shallow searches
        };
        
        // Check for game end conditions
        if let Some(terminal_score) = self.check_terminal_position(&board, depth_left_before) {
            self.stats.terminal_nodes += 1;
            return terminal_score;
        }

        // Leaf node evaluation
        if depth_left_before == 0 {
            self.stats.evaluations += 1;
            return match chromosome {
                Some(chr) => Evaluator::evaluate_with_chromosome(board, chr),
                None => Evaluator::evaluate(board),
            };
        }

        let mut beta = beta_before;
        let original_beta = beta_before;
        let mut moves = self.get_ordered_moves(&board);
        
        // Order TT move first if available
        if let Some(tt_mv) = tt_move {
            if let Some(pos) = moves.iter().position(|&mv| mv == tt_mv) {
                moves.swap(0, pos);
            }
        }
        
        let mut best_move = None;

        for mov in moves {
            let new_board = board.make_move_new(mov);
            let score = self.alpha_beta_max(new_board, alpha, beta, depth_left_before - 1, chromosome);

            if score <= alpha {
                self.stats.cutoffs += 1;
                // Store alpha cutoff in TT (only for deeper searches)
                if depth_left_before >= 2 {
                    self.transposition_table.store(board_hash, depth_left_before, NodeType::UpperBound, alpha, Some(mov));
                }
                return alpha; // Alpha cutoff
            }
            if score < beta {
                beta = score;
                best_move = Some(mov);
            }
        }
        
        // Store result in transposition table (only for deeper searches)
        if depth_left_before >= 2 {
            let node_type = if beta >= original_beta {
                NodeType::LowerBound // All moves failed high
            } else {
                NodeType::Exact // PV node
            };
            self.transposition_table.store(board_hash, depth_left_before, node_type, beta, best_move);
        }

        beta
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
