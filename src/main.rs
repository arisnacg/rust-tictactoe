#![allow(unused)]

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::process::Command;
use std::{fmt::format, usize};

use tabled::{
    builder::Builder,
    settings::{object::Rows, Alignment, Modify, Style},
};

const TOTAL_ROWS: usize = 3;
const TOTAL_COLUMNS: usize = 3;
const MAX_FILL: usize = TOTAL_ROWS * TOTAL_COLUMNS;

fn main() {
    let mut board = create_board();

    let mut game_end = false;
    clearscreen();
    println!("[*] WELCOME TO TICTACTOE GAME [*]");
    let human_char = ask_player_char();
    let ai_char = (if human_char == 'X' { 'O' } else { 'X' });
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

fn clearscreen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn move_array_to_num(move_arr: [usize; 2], board_rows: usize) -> usize {
    return move_arr[0] * board_rows + move_arr[1] + 1;
}

fn move_num_to_array(num: usize, board_rows: usize) -> [usize; 2] {
    let i: usize = (num - 1) / board_rows;
    let j: usize = (num - 1) % board_rows;
    return [i, j];
}

fn ask_player_move(board: Vec<Vec<char>>, human_char: char) -> [usize; 2] {
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

fn write_ai_log(data: String) {
    // Open the file in append mode
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("ai.log")
        .expect("Failed to open file");

    // Write the data to the file
    if let Err(err) = file.write_all(data.as_bytes()) {
        eprintln!("Error writing to file: {}", err);
    }
}

fn minimax(
    board: &mut Vec<Vec<char>>,
    is_maximizing: bool,
    depth: isize,
    ai_char: char,
    human_char: char,
) -> isize {
    let result = check_winner(board.clone());
    // print_board(board.to_vec());
    // println!("Depth: {} ", depth);
    if result != ' ' {
        if result == 'D' {
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
                if board[i][j] == ' ' {
                    board[i][j] = ai_char;
                    let score = minimax(board, false, depth + 1, ai_char, human_char);
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
                    let score = minimax(board, true, depth + 1, ai_char, human_char);
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

fn ai_best_move(board: &mut Vec<Vec<char>>, ai_char: char, human_char: char) -> [usize; 2] {
    let x_length = board.len();
    let y_length = board[0].len();
    let mut best_score = -100;
    let mut best_move: [usize; 2] = Default::default();
    write_ai_log("AI's Possible moves:\n".to_string());
    for i in 0..x_length {
        for j in 0..y_length {
            if board[i][j] == ' ' {
                board[i][j] = ai_char;
                let score = minimax(board, false, 1, ai_char, human_char);
                board[i][j] = ' ';
                let move_num = move_array_to_num([i, j], x_length);
                write_ai_log(format!("- {} -> Score: {}\n", move_num, score));
                if score > best_score {
                    best_score = score;
                    best_move = [i, j];
                }
            }
        }
    }
    write_ai_log(format!(
        "AI's Best move: {}\n\n",
        move_array_to_num(best_move, x_length)
    ));
    return best_move;
}

fn ask_player_char() -> char {
    println!("[*] First/second (X/O)?:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let character: char = input.trim().chars().next().expect("No input provided.");
    return character.to_ascii_uppercase();
}

fn check_winner(board: Vec<Vec<char>>) -> char {
    if is_win(board.clone(), 'X') {
        return 'X';
    } else if is_win(board.clone(), 'O') {
        return 'O';
    }

    let mut filled_count = 0;
    let x_length = board.len();
    let y_length = board[0].len();
    for i in 0..x_length {
        for j in 0..y_length {
            if board[i][j] != ' ' {
                filled_count += 1;
            }
        }
    }
    if filled_count == x_length * y_length {
        return 'D';
    }
    return ' ';
}

fn is_win(board: Vec<Vec<char>>, player_char: char) -> bool {
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

    let mut builder = Builder::default();
    for i in 0..x_length {
        let mut row: Vec<char> = Vec::new();
        for j in 0..y_length {
            if board[i][j] == ' ' {
                let box_num = (i * x_length + j + 1);
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

// create tic tac toe board
fn create_board() -> Vec<Vec<char>> {
    let mut array: Vec<Vec<char>> = Vec::new();
    for _ in 0..TOTAL_ROWS {
        let row: Vec<char> = vec![' '; TOTAL_COLUMNS];
        array.push(row);
    }
    return array;
}
