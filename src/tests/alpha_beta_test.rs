#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use chess::{Board, BoardStatus, Color};

    use crate::alpha_beta_algorithm::{AlgorithmTraits, AlphaBetaAlgorithm};

    #[test]
    fn checkmate1() {
        let board: Board = Board::from_str("7K/5k2/8/8/8/8/8/r7 w - - 0 1").unwrap(); // 1 llä
        let required_moves = game_loop(board, 2);

        assert_eq!(required_moves, 2)
    }

    #[test]
    fn checkmate2() {
        let board = Board::from_str("7k/8/5K2/8/8/8/8/5R2 w - - 0 1").unwrap(); //2:lla
        let required_moves = game_loop(board, 4);

        assert_eq!(required_moves, 3)
    }

    #[test]
    fn checkmate4() {
        let board = Board::from_str("8/8/8/5k1K/6r1/8/8/8 w - - 10 6").unwrap(); //4:lla
        let required_moves = game_loop(board, 8);

        assert_eq!(required_moves, 8)
    }

    #[test]
    fn checkmate5() {
        let board: Board = Board::from_str("8/8/4k1K1/8/6r1/8/8/8 w - - 8 5").unwrap(); //5:lla
        let required_moves = game_loop(board, 10);

        assert_eq!(required_moves, 10)
    }
    #[test]
    fn checkmate6() {
        let board = Board::from_str("8/8/4k1K1/8/r7/8/8/8 b - - 7 4").unwrap(); //6:lla
        let required_moves = game_loop(board, 12);

        assert_eq!(required_moves, 12)
    }

    #[test]
    fn checkmate8() {
        let board = Board::from_str("8/6K1/8/4k3/8/8/8/r7 w - - 0 1").unwrap(); // shakkimatti 8:lla
        let required_moves = game_loop(board, 16);

        assert_eq!(required_moves, 16)
    }

    fn game_loop(mut board: Board, calculation_half_depth: i32) -> i32 {
        let mut move_amount = 0;
        let mut alg = AlphaBetaAlgorithm;
        loop {
            let ai_move = alg.get_best_move(board, calculation_half_depth).unwrap();
            board = board.make_move_new(ai_move);
            move_amount += 1;

            let game_result: BoardStatus = board.status();

            if game_result != BoardStatus::Ongoing {
                println!("Game ended");
                if game_result == BoardStatus::Checkmate {
                    let winner = match board.side_to_move() {
                        Color::Black => "White",
                        Color::White => "Black",
                    };
                    println!("Checkmate: {} wins", winner);
                } else {
                    println!("Stalemate");
                }
                return move_amount;
            }
        }
    }
}
