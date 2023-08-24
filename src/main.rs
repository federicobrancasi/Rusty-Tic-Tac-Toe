mod utils;
use utils::board::*;
use utils::game::*;
use utils::player::*;

fn main() {
    const TOTAL_ROWS: usize = 3;
    const TOTAL_COLUMNS: usize = 3;

    clearscreen();

    let mut board = create_board(TOTAL_ROWS, TOTAL_COLUMNS);

    let human_char = ask_player_char();
    let human_move = ask_player_move(board.clone(), human_char);
    fill_box(&mut board, human_move[0], human_move[1], human_char);

    print_board(board.clone());
}












