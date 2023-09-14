mod utils;
use utils::board::*;
use utils::game::*;
use utils::player::*;

fn main() {
    const TOTAL_ROWS: usize = 3;
    const TOTAL_COLUMNS: usize = 3;

    clearscreen();

    let mut board = create_board(TOTAL_ROWS, TOTAL_COLUMNS);
    let human_char = 'X';
    let ai_char = 'O';

    fill_box(&mut board, 0, 0, human_char);
    fill_box(&mut board, 1, 1, ai_char);
    fill_box(&mut board, 0, 1, human_char);
    fill_box(&mut board, 0, 2, ai_char);
    fill_box(&mut board, 1, 0, human_char);

    print_board(board.clone());

    let best_move_arr = ai_best_move(&mut board, ai_char, human_char);
    let best_move_num = move_array_to_num(best_move_arr, TOTAL_ROWS);

    println!("AI's best move is {}", best_move_num);

    fill_box(&mut board, best_move_arr[0], best_move_arr[1], ai_char);
    print_board(board.clone());

    if is_win(board, ai_char) {
        println!("{} Win!", ai_char);
    }
}

pub fn minmax(board: &mut Vec<Vec<char>>,
              is_maximizing: bool,
              depth: isize,
              ai_char: char,
              human_char: char,
              ) -> isize {
    let result = check_winner(board.clone());

    if result != ' '{
        if result == 'D'{
            return 0;
        } else if result == human_char {
            return -100;
        } else {
            return 100;
        }
    }

    let x_length = board.len();
    let y_length = board[0].len();

    if is_maximizing {
        let mut best_score = -100;
        for i in 0..x_length {
            for j in 0..y_length {
                if board[i][j] == ' '{
                    board[i][j] = ai_char;
                    let score = minmax(board, false, depth + 1, ai_char, human_char);
                    board[i][j] = ' ';
                    if score > best_score {
                        best_score = score;
                    }
                }
            }
        }
        return best_score - depth;
    } else {
        let mut best_score = 100;
        for i in 0..x_length {
            for j in 0..y_length {
                if board[i][j] == ' ' {
                    board[i][j] = human_char;
                    let score = minmax(board, true, depth + 1, ai_char, human_char);
                    board[i][j] = ' ';
                    if score < best_score {
                        best_score = score;
                    }
                }
            }
        }
        return best_score - depth;
    }
}

pub fn ai_best_move(board: &mut Vec<Vec<char>>, ai_char: char, human_char: char) -> [usize; 2] {
    let x_length = board.len();
    let y_length = board[0].len();
    let mut best_score = -100;
    let mut best_move: [usize; 2] = Default::default();
    for i in 0..x_length {
        for j in 0..y_length {
            if board[i][j] == ' ' {
                board[i][j] = ai_char;
                let score = minmax(board, false, 1, ai_char, human_char);
                board[i][j] = ' ';
                let move_num = move_array_to_num([i, j], x_length);
                if score > best_score {
                    best_score = score;
                    best_move =  [i, j];
                }
            }
        }
    }
    return best_move;
}

