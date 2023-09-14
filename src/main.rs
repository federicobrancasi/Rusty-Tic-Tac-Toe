mod utils;
use utils::board::*;
use utils::game::*;
use utils::player::*;

fn main() {
    const TOTAL_ROWS: usize = 3;
    const TOTAL_COLUMNS: usize = 3;
    const MAX_FILL: usize = TOTAL_ROWS * TOTAL_COLUMNS;
    let mut board = create_board(TOTAL_ROWS, TOTAL_COLUMNS);

    clearscreen();
    println!("[*] WELCOME TO TICTACTOE GAME [*]");
    let human_char = ask_player_char();
    let ai_char = if human_char == 'X' { 'O' } else { 'X' };
    let mut filled_box_count = 0;
    let mut winner = ' ';
    let mut ai_last_move = 0;

    while filled_box_count < MAX_FILL {
        if ai_char == 'X' {
            clearscreen();
            let ai_move = ai_best_move(&mut board, ai_char, human_char);
            fill_box(&mut board, ai_move[0], ai_move[1], ai_char);
            filled_box_count += 1;
            print_board(board.clone());
            println!(
                "[+] AI move : X -> {}",
                move_array_to_num(ai_move, TOTAL_ROWS)
                );
            if is_win(board.clone(), ai_char) {
                winner = ai_char;
                break;
            }
            let human_move = ask_player_move(board.clone(), human_char);
            fill_box(&mut board, human_move[0], human_move[1], human_char);
            filled_box_count += 1;
            if is_win(board.clone(), human_char) {
                winner = human_char;
                break;
            }
        } else {
            clearscreen();
            print_board(board.clone());
            if ai_last_move == 0 {
                println!("[*] AI is waiting your move...");
            } else {
                println!("[+] AI move: O -> {}", ai_last_move);
            }
            let human_move = ask_player_move(board.clone(), human_char);
            fill_box(&mut board, human_move[0], human_move[1], human_char);
            filled_box_count += 1;
            if is_win(board.clone(), human_char) {
                winner = human_char;
                break;
            }
            let ai_move = ai_best_move(&mut board, ai_char, human_char);
            fill_box(&mut board, ai_move[0], ai_move[1], ai_char);
            filled_box_count += 1;
            if is_win(board.clone(), ai_char) {
                winner = ai_char;
                break;
            }
            ai_last_move = move_array_to_num(ai_move, TOTAL_ROWS);
        }
    }
    clearscreen();
    if winner == human_char {
        println!("[*] YOU ({}) WIN [*]", human_char);
    } else if winner == ai_char {
        println!("[*] YOU ({}) LOSE! [*]", human_char);
    } else {
        println!("[*] DRAW! [*]");
    }
    print_board(board.clone());
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
                if score > best_score {
                    best_score = score;
                    best_move =  [i, j];
                }
            }
        }
    }
    return best_move;
}

