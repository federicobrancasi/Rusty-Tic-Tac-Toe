use std::io;

pub fn ask_player_char() -> char {
    println!("[*] First/second (X/O)?:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let character: char = input.trim().chars().next().expect("No input provided.");
    return character.to_ascii_uppercase();
}

pub fn ask_player_move(board: Vec<Vec<char>>, human_char: char) -> [usize; 2] {
    loop {
        println!("[+] Your move {} -> (1-9)?: ", human_char);
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let player_move: usize = input.trim().parse().expect("Please enter a number");
        let player_move_array = move_num_to_array(player_move, board.len());
        if board[player_move_array[0]][player_move_array[1]] != ' ' {
            println!("[!] Invalid: {} already filled", player_move)
        } else {
            return player_move_array;
        }
    }
}

pub fn move_array_to_num(move_arr: [usize; 2], board_rows: usize) -> usize {
    return move_arr[0] * board_rows + move_arr[1] + 1;
}

fn move_num_to_array(num: usize, board_rows: usize) -> [usize; 2] {
    let i: usize = (num - 1) / board_rows;
    let j: usize = (num - 1) % board_rows;
    return [i, j];
}


