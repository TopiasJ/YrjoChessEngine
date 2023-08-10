use chess::{Board, Color, Piece};
pub struct Evaluator;

impl Evaluator {
    #[inline]
    pub fn evaluate(board_state: Board) -> i32 {
        Self::get_board_value_bitboard(board_state)
    }

    #[inline]
    fn get_board_value_bitboard(board: Board) -> i32 {
        let bit_board_white = *board.color_combined(Color::White);
        let bit_board_black = *board.color_combined(Color::Black);
        //println!("bitboard {}", bit_board_white);
        let mut current_score = 0;
        for square in bit_board_white {
            let pie = board.piece_on(square).unwrap();
            current_score += Self::get_piece_value2(pie);
        }
        for square in bit_board_black {
            let pie = board.piece_on(square).unwrap();
            current_score -= Self::get_piece_value2(pie);
        }
        current_score
    }
    #[inline]
    fn get_piece_value2(pie: Piece) -> i32 {
        match pie {
            Piece::Pawn => 100,
            Piece::Rook => 500,
            Piece::Bishop => 300,
            Piece::Knight => 300,
            Piece::Queen => 900,
            Piece::King => 10000,
        }
    }
}
