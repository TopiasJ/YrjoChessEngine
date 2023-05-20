use chess::{Board, Color, Piece};
pub struct Evaluator;

impl Evaluator {
    #[inline]
    pub fn evaluate(board_state: Board) -> i32 {
        let boardstr = board_state.to_string();
        let binding: Vec<&str> = boardstr.split_whitespace().collect();
        let current_score = Self::get_board_value(binding[0].to_string());
        return current_score;
    }
    #[inline]
    pub fn evaluate2(board_state: Board) -> i32 {
        let current_score2 = Self::get_board_value_bitboard(board_state);
        return current_score2;
    }
    #[inline]
    fn get_piece_value(mut piece: char) -> i32 {
        let is_white: bool = piece.is_uppercase();
        piece = piece.to_ascii_lowercase();
        let value: i32;
        if piece == 'p' {
            value = 100;
        } else if piece == 'r' {
            value = 500;
        } else if piece == 'n' {
            value = 300;
        } else if piece == 'b' {
            value = 300;
        } else if piece == 'q' {
            value = 900;
        } else if piece == 'k' {
            value = 10000;
        } else {
            value = 0;
        }
        if is_white {
            return value;
        } else {
            return -value;
        }
    }
    #[inline]
    fn get_board_value(board_state: String) -> i32 {
        let mut current_score = 0;
        for square in board_state.chars() {
            current_score += Self::get_piece_value(square);
        }
        return current_score;
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
        return current_score;
    }
    #[inline]
    fn get_piece_value2(pie: Piece) -> i32 {
        if pie == Piece::Pawn {
            return 100;
        } else if pie == Piece::Rook {
            return 500;
        } else if pie == Piece::Bishop {
            return 300;
        } else if pie == Piece::Knight {
            return 300;
        } else if pie == Piece::Queen {
            return 900;
        } else if pie == Piece::King {
            return 10000;
        } else {
            return 0;
        }
    }
}
