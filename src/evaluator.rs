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
        let bit_board_white = *board.color_combined(Color::White);
        let bit_board_black = *board.color_combined(Color::Black);
        //println!("bitboard {}", bit_board_white);
        let mut current_score = 0;
        for square in bit_board_white {
            let pie = board.piece_on(square).unwrap();
            current_score += Self::get_piece_value(pie, chromosome);
        }
        for square in bit_board_black {
            let pie = board.piece_on(square).unwrap();
            current_score -= Self::get_piece_value(pie, chromosome);
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
