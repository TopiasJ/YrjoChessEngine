use crate::chromosome::Chromosome;
use crate::evaluator::Evaluator;
use chess::{Board, ChessMove, Color, MoveGen, EMPTY};
use rand::Rng;

pub trait AlgorithmTraits {
    fn get_best_move(&mut self, board: Board, depth: i32) -> Option<ChessMove>;
    fn get_best_move_with_chromosome(&mut self, board: Board, depth: i32, chromosome: &Chromosome) -> Option<ChessMove>;
}

pub struct AlphaBetaAlgorithm;

impl AlgorithmTraits for AlphaBetaAlgorithm {
    fn get_best_move(&mut self, board: Board, depth: i32) -> Option<ChessMove> {
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
    fn calc_one_move(&self, best_moves: &mut Vec<(ChessMove, i32)>, a_move: ChessMove, test_game: Board, depth: i32, chromosome: Option<&Chromosome>) {
        let new_board = test_game.make_move_new(a_move);
        let result: i32 = match new_board.side_to_move() {
            Color::White => self.alpha_beta_max(new_board, -999999, 999999, depth, chromosome),
            Color::Black => self.alpha_beta_min(new_board, -999999, 999999, depth, chromosome),
        };
        best_moves.push((a_move, result));
    }

    fn alpha_beta_max(&self, board: Board, alpha_before: i32, beta: i32, depth_left_before: i32, chromosome: Option<&Chromosome>) -> i32 {
        // Check for game end conditions
        if let Some(terminal_score) = self.check_terminal_position(&board, depth_left_before) {
            return terminal_score;
        }

        // Leaf node evaluation
        if depth_left_before == 0 {
            return match chromosome {
                Some(chr) => Evaluator::evaluate_with_chromosome(board, chr),
                None => Evaluator::evaluate(board),
            };
        }

        let mut alpha = alpha_before;
        let moves = self.get_ordered_moves(&board);

        for mov in moves {
            let new_board = board.make_move_new(mov);
            let score = self.alpha_beta_min(new_board, alpha, beta, depth_left_before - 1, chromosome);

            if score >= beta {
                return beta; // Beta cutoff
            }
            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }

    fn alpha_beta_min(&self, board: Board, alpha: i32, beta_before: i32, depth_left_before: i32, chromosome: Option<&Chromosome>) -> i32 {
        // Check for game end conditions
        if let Some(terminal_score) = self.check_terminal_position(&board, depth_left_before) {
            return terminal_score;
        }

        // Leaf node evaluation
        if depth_left_before == 0 {
            return match chromosome {
                Some(chr) => Evaluator::evaluate_with_chromosome(board, chr),
                None => Evaluator::evaluate(board),
            };
        }

        let mut beta = beta_before;
        let moves = self.get_ordered_moves(&board);

        for mov in moves {
            let new_board = board.make_move_new(mov);
            let score = self.alpha_beta_max(new_board, alpha, beta, depth_left_before - 1, chromosome);

            if score <= alpha {
                return alpha; // Alpha cutoff
            }
            if score < beta {
                beta = score;
            }
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
    fn get_ordered_moves(&self, board: &Board) -> Vec<ChessMove> {
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
