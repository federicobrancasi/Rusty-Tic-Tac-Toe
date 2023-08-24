mod utils;
use utils::board::*;
use utils::game::*;

fn main() {
    const TOTAL_ROWS: usize = 3;
    const TOTAL_COLUMNS: usize = 3;

    clearscreen();

    let mut board = create_board(TOTAL_ROWS, TOTAL_COLUMNS);

    fill_box(&mut board, 1, 1, 'X');
    fill_box(&mut board, 0, 0, 'X');
    fill_box(&mut board, 2, 2, 'X');

    print_board(board.clone());

    let winner = check_winner(board);

    println!("The winner is {}", winner);
}












