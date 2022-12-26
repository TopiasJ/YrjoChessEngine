use chess::Board;
pub struct Visualizer;
impl Visualizer{
    pub fn visualize_board(board:Board ){
        let binding2 = board.to_string();
        let binding:Vec<&str>= binding2.split_whitespace().collect();
        let board_rows = binding[0].split("/");
        let mut row_number = 8;
        for row in board_rows {
            let row_squares = row.chars();
            //rowSquares = [char for char in row]
            let mut str_row = String::new();
            for square in row_squares {
                str_row += &Visualizer::get_piece_symbol(square);
            }
            println!("{} {}", (row_number.to_string()), str_row);
            row_number -= 1
        }
    
        println!("  a b c d e f g h");
    }
    fn get_piece_symbol(code:char) -> String {
        if code == 'p'{
            return "\u{265F} ".to_string();
        }else if code == 'r'{
            return "\u{265C} ".to_string();}
        else if code == 'n'{
            return "\u{265E} ".to_string();
        }else if code == 'b'{
            return "\u{265D} ".to_string();
        }else if code == 'k'{
            return "\u{265A} ".to_string();
        }else if code == 'q'{
            return "\u{265B} ".to_string();
        }else if code == 'P'{
            return "\u{2659} ".to_string(); 
        }else if code == 'R'{
            return "\u{2656} ".to_string(); 
        }else if code == 'N' {
            return "\u{2658} ".to_string();  
        }else if code == 'B' {
            return "\u{2657} ".to_string();
        }else if code == 'K'{
            return "\u{2654} ".to_string(); 
        }else if code == 'Q'{
            return "\u{2655} ".to_string();  
        }else { // it is a number instead
            let mut empty_spaces = String::new();
            for _ in 0..(code.to_string().parse::<i32>().unwrap()){
                empty_spaces += "  ";
            }
            return empty_spaces;
        }

    }
}