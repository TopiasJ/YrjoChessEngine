use crate::evaluator::Evaluator;
use chess::{Board, ChessMove, Color, MoveGen, EMPTY};
use rand::Rng;

pub trait AlgorithmTraits {
    fn get_best_move(&mut self, board: Board, depth: i32) -> Option<ChessMove>;
}

pub struct AlphaBetaAlgorithm;

impl AlgorithmTraits for AlphaBetaAlgorithm {
    fn get_best_move(&mut self, board: Board, depth: i32) -> Option<ChessMove> {
        let mut best_moves: Vec<(ChessMove, i32)> = Vec::new();
        let mut moves_iterable = MoveGen::new_legal(&board);

        for mov in &mut moves_iterable {
            self.calc_one_move(&mut best_moves, mov, board, depth);
        }
        best_moves.sort_by_key(|k| k.1);

        let selected_index =
            get_random_from_multiple_best_moves(&best_moves, board.side_to_move())?;
        let selected_move = match board.side_to_move() {
            Color::White => best_moves[best_moves.len() - 1 - selected_index],
            Color::Black => best_moves[selected_index],
        };
        let color: String = match board.side_to_move() {
            Color::White => "White".to_string(),
            Color::Black => "Black".to_string(),
        };
        println!(
            "value for selected move for {0}: {1}",
            color, selected_move.1
        );
        return Some(selected_move.0);
    }
}
fn get_random_from_multiple_best_moves(
    best_moves: &Vec<(ChessMove, i32)>,
    color: Color,
) -> Option<usize> {
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
    let mut rng = rand::thread_rng();
    let selected_index: i32 = rng.gen_range(0..amount_of_equal_moves); // doest not include last
    return Some(selected_index as usize);
}

impl AlphaBetaAlgorithm {
    #[inline]
    fn calc_one_move(
        &self,
        best_moves: &mut Vec<(ChessMove, i32)>,
        a_move: ChessMove,
        test_game: Board,
        depth: i32,
    ) {
        let new_board = test_game.make_move_new(a_move);
        let result: i32 = if new_board.side_to_move() == Color::White {
            self.alpha_beta_max(new_board, -999999, 999999, depth)
        } else {
            self.alpha_beta_min(new_board, -999999, 999999, depth)
        };
        best_moves.push((a_move, result));
    }
    fn alpha_beta_max(
        &self,
        board: Board,
        alpha_before: i32,
        beta: i32,
        depth_left_before: i32,
    ) -> i32 {
        let mut moves_iterable = MoveGen::new_legal(&board); //get all legal moves
        let depthleft = depth_left_before;
        let mut alpha = alpha_before;
        if moves_iterable.len() == 0 {
            //game ended
            if board.checkers() == &EMPTY {
                return 0;
            } else {
                return match board.side_to_move() {
                    Color::White => -9999 - depth_left_before,
                    Color::Black => 9999 + depth_left_before,
                };
            }
        }

        if depthleft == 0 {
            return Evaluator::evaluate2(board);
        }
        let targets = board.color_combined(!board.side_to_move());
        moves_iterable.set_iterator_mask(*targets);

        for mov in &mut moves_iterable {
            let new_board = board.make_move_new(mov);
            let score = self.alpha_beta_min(new_board, alpha, beta, depthleft - 1);

            if score >= beta {
                return beta; //fail hard beta - cutoff
            }
            if score > alpha {
                alpha = score; // alpha acts like max in MiniMax
            }
        }

        moves_iterable.set_iterator_mask(!EMPTY);

        for mov in &mut moves_iterable {
            let new_board = board.make_move_new(mov);
            let score = self.alpha_beta_min(new_board, alpha, beta, depthleft - 1);

            if score >= beta {
                return beta; //fail hard beta - cutoff
            }
            if score > alpha {
                alpha = score; // alpha acts like max in MiniMax
            }
        }

        return alpha;
    }

    fn alpha_beta_min(
        &self,
        board: Board,
        alpha: i32,
        beta_before: i32,
        depth_left_before: i32,
    ) -> i32 {
        let mut moves_iterable = MoveGen::new_legal(&board); //get all legal moves
        let mut depthleft = depth_left_before;
        let mut beta = beta_before;
        if moves_iterable.len() == 0 {
            //game ended
            if board.checkers() == &EMPTY {
                return 0;
            } else {
                return match board.side_to_move() {
                    Color::White => -9999 - depth_left_before,
                    Color::Black => 9999 + depth_left_before,
                };
            }
        }

        if depthleft == 0 {
            return Evaluator::evaluate2(board);
        }
        let targets = board.color_combined(!board.side_to_move());
        moves_iterable.set_iterator_mask(*targets);

        for mov in &mut moves_iterable {
            let new_board = board.make_move_new(mov);
            let score = self.alpha_beta_max(new_board, alpha, beta, depthleft - 1);
            if score <= alpha {
                return alpha; // fail hard alpha - cutoff
            }
            if score < beta {
                beta = score; // beta acts like min in MiniMax
            }
        }

        moves_iterable.set_iterator_mask(!EMPTY);

        for mov in &mut moves_iterable {
            let new_board = board.make_move_new(mov);
            let score = self.alpha_beta_max(new_board, alpha, beta, depthleft - 1);
            if score <= alpha {
                return alpha; // fail hard alpha - cutoff
            }
            if score < beta {
                beta = score; // beta acts like min in MiniMax
            }
        }
        return beta;
    }
}
