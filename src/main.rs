use tabled::builder::Builder;
use tabled::settings::Style;

fn clearscreen(){
    print!("\x1B[2J\x1B[1;1H");
}

fn fill_box(board: &mut Vec<Vec<char>>, x: usize, y: usize, player_char: char){
    if let Some(row) = board.get_mut(x){
        if let Some(element) = row.get_mut(y){
            *element = player_char;
        }
    }
}

fn print_board(board: Vec<Vec<char>>){
    let x_length = board.len();
    let y_lenght = board[0].len();

    let mut builder = Builder::default();
    
    for i in 0..x_length {
        let mut row: Vec<char> = Vec::new();
        for j in 0..y_lenght{
            if board[i][j] == ' '{
                let box_num = i * x_length + j + 1;
                let box_num_char = (b'0' + box_num as u8) as char;
                row.push(box_num_char);
            } else {
                row.push(board[i][j]);
            }
        }
        builder.push_record(row);
    }
    let table = builder.build().with(Style::modern()).to_string();
    println!("{}", table);

}

fn create_board(total_rows: usize, total_columns: usize) -> Vec<Vec<char>> {
    let mut array: Vec<Vec<char>> = Vec::new();
    for _ in 0..total_rows {
        let row: Vec<char> = vec![' '; total_columns];
        array.push(row);
    }
    return array;
}



fn main() {
    const TOTAL_ROWS: usize = 3;
    const TOTAL_COLUMNS: usize = 3;
    clearscreen();
    let mut board = create_board(TOTAL_ROWS, TOTAL_COLUMNS);
    print_board(board.clone());
    fill_box(&mut board, 1, 1, 'X');
    print_board(board.clone());
}












