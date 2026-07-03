use crate::chromosome::Chromosome;
use chess::{Board, Color, Piece};
pub struct Evaluator;

impl Evaluator {
    #[inline]
    pub fn evaluate(board_state: Board) -> i32 {
        Self::get_board_value_bitboard(board_state, None)
    }

    #[inline]
    pub fn evaluate_with_chromosome(board_state: Board, chromosome: &Chromosome) -> i32 {
        Self::get_board_value_bitboard(board_state, Some(chromosome))
    }

    #[inline]
    fn get_board_value_bitboard(board: Board, chromosome: Option<&Chromosome>) -> i32 {
        const PIECES: [Piece; 6] = [Piece::Pawn, Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen, Piece::King];

        let bit_board_white = *board.color_combined(Color::White);
        let bit_board_black = *board.color_combined(Color::Black);
        let mut current_score = 0;
        // Material only, so counting each piece bitboard with popcount is enough —
        // no need to look up the piece on every occupied square.
        for piece in PIECES {
            let piece_bb = *board.pieces(piece);
            let value = Self::get_piece_value(piece, chromosome);
            let count_difference = (piece_bb & bit_board_white).popcnt() as i32 - (piece_bb & bit_board_black).popcnt() as i32;
            current_score += count_difference * value;
        }
        current_score
    }
    #[inline]
    fn get_piece_value(pie: Piece, chromosome: Option<&Chromosome>) -> i32 {
        match chromosome {
            Some(chr) => match pie {
                Piece::Pawn => chr.pawn_value,
                Piece::Rook => chr.rook_value,
                Piece::Bishop => chr.bishop_value,
                Piece::Knight => chr.knight_value,
                Piece::Queen => chr.queen_value,
                Piece::King => chr.king_value,
            },
            None => match pie {
                Piece::Pawn => 100,
                Piece::Rook => 500,
                Piece::Bishop => 300,
                Piece::Knight => 300,
                Piece::Queen => 900,
                Piece::King => 10000,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn starting_position_is_balanced() {
        assert_eq!(Evaluator::evaluate(Board::default()), 0);
    }

    #[test]
    fn missing_knight_costs_its_default_value() {
        // White is missing the b1 knight
        let board = Board::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R1BQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(Evaluator::evaluate(board), -300);
    }

    #[test]
    fn chromosome_values_drive_the_score() {
        let chromosome = Chromosome {
            pawn_value: 1,
            knight_value: 3,
            bishop_value: 3,
            rook_value: 5,
            queen_value: 9,
            king_value: 0,
        };
        // Black is missing the d8 queen
        let board = Board::from_str("rnb1kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(Evaluator::evaluate_with_chromosome(board, &chromosome), 9);
        assert_eq!(Evaluator::evaluate(board), 900);
    }
}
