#![allow(unused)]

use std::{fmt::format, usize};

fn main() {
    const TOTAL_ROWS: usize = 3;
    const TOTAL_COLUMNS: usize = 3;
    let mut board = create_board(TOTAL_ROWS, TOTAL_COLUMNS);

    fill_box(&mut board, 0, 0, 'O');
    fill_box(&mut board, 1, 0, 'X');
    fill_box(&mut board, 1, 1, 'O');
    fill_box(&mut board, 1, 2, 'X');
    fill_box(&mut board, 2, 2, 'O');
    print_board(board.clone());
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
