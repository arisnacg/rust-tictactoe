#![allow(unused)]

use std::io;
use std::{fmt::format, usize};

fn main() {
    const TOTAL_ROWS: usize = 3;
    const TOTAL_COLUMNS: usize = 3;
    const MAX_FILL: usize = TOTAL_ROWS * TOTAL_COLUMNS;
    let mut board = create_board(TOTAL_ROWS, TOTAL_COLUMNS);
    // let mut game_end = false;
    // let player1_char = ask_player_char();
    // let player2_char = (if player1_char == 'X' { 'O' } else { 'X' });
    let player_move = ask_player_move();
    let arr_move = move_num_to_array(player_move, TOTAL_ROWS);
    let num_move = move_array_to_num(arr_move, TOTAL_ROWS);
    println!(
        "player move: {} -> {}, {} -> {}",
        player_move, arr_move[0], arr_move[1], num_move
    );
}

fn move_array_to_num(move_arr: [usize; 2], board_rows: usize) -> usize {
    return move_arr[0] * board_rows + move_arr[1] + 1;
}

fn move_num_to_array(num: usize, board_rows: usize) -> [usize; 2] {
    let i: usize = (num - 1) / board_rows;
    let j: usize = (num - 1) % board_rows;
    return [i, j];
}

fn ask_player_move() -> usize {
    println!("Your move (1-9): ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let player_move: usize = input.trim().parse().expect("Please enter a number");
    return player_move;
}

fn ai_move(board: Vec<Vec<char>>) -> [usize; 2] {
    let x_length = board.len();
    let y_length = board[0].len();
    for i in 0..x_length {
        for j in 0..y_length {
            if board[i][j] == ' ' {
                return [i, j];
            }
        }
    }
    return [0, 0];
}

fn ask_player_char() -> char {
    println!("Chose X or O :");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let character: char = input.trim().chars().next().expect("No input provided.");
    return character.to_ascii_uppercase();
}

fn is_win(board: &mut Vec<Vec<char>>, player_char: char) -> bool {
    let x_length = board.len();
    for i in 0..x_length {
        // check rows
        if board[i][0] == player_char && board[i][1] == player_char && board[i][2] == player_char {
            return true;
        }
        // check columns
        if board[0][i] == player_char && board[1][i] == player_char && board[2][i] == player_char {
            return true;
        }
    }
    // check diagonals
    if board[0][0] == player_char && board[1][1] == player_char && board[2][2] == player_char {
        return true;
    }
    if board[0][2] == player_char && board[1][1] == player_char && board[2][0] == player_char {
        return true;
    }
    // no win condition found
    return false;
}

// add char into board box
fn fill_box(board: &mut Vec<Vec<char>>, x: usize, y: usize, player_char: char) {
    if let Some(row) = board.get_mut(x) {
        if let Some(element) = row.get_mut(y) {
            *element = player_char;
        }
    }
}

// print tic tac toe board
fn print_board(board: Vec<Vec<char>>) {
    let x_length = board.len();
    let y_length = board[0].len();

    // print row index guide top
    let mut row_guide = String::new();
    row_guide.push(' '); // for column guide
    for i in 0..x_length {
        let column_str = format!(" {}", i + 1);
        row_guide.push_str(&column_str)
    }
    println!("{}", row_guide);
    // print each board row
    for i in 0..x_length {
        let mut row_str = String::new();
        row_str.push_str(&(i + 1).to_string());
        for j in 0..y_length {
            let column_str = format!(" {}", board[i][j]);
            row_str.push_str(&column_str)
        }
        row_str.push_str(&format!(" {}", i + 1));
        println!("{}", row_str);
    }
    // print row index guide bottom
    println!("{}", row_guide);
    println!();
}

// create tic tac toe board
fn create_board(total_rows: usize, total_colums: usize) -> Vec<Vec<char>> {
    let mut array: Vec<Vec<char>> = Vec::new();
    for _ in 0..total_rows {
        let row: Vec<char> = vec![' '; total_colums];
        array.push(row);
    }
    return array;
}
