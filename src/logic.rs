use term_table::table_cell::{Alignment, TableCell};
use term_table::{TableBuilder, TableStyle};
use term_table::row::Row;
use figlet_rs::*;
use colored::*;

fn draw_table_board(board: &mut Vec<char>) {
    let table = TableBuilder::new().style(TableStyle::extended()).rows(
        vec![
            Row::new(vec![
                TableCell::new_with_alignment("1", 1, Alignment::Center),
                TableCell::new_with_alignment("2", 1, Alignment::Center),
                TableCell::new_with_alignment("3", 1, Alignment::Center),
                TableCell::new_with_alignment("4", 1, Alignment::Center),
                TableCell::new_with_alignment("5", 1, Alignment::Center),
                TableCell::new_with_alignment("6", 1, Alignment::Center),
                TableCell::new_with_alignment("7", 1, Alignment::Center),
            ]),
            Row::new(vec![
                TableCell::new_with_alignment(board[0], 1, Alignment::Left),
                TableCell::new_with_alignment(board[1], 1, Alignment::Left),
                TableCell::new_with_alignment(board[2], 1, Alignment::Left),
                TableCell::new_with_alignment(board[3], 1, Alignment::Center),
                TableCell::new_with_alignment(board[4], 1, Alignment::Right),
                TableCell::new_with_alignment(board[5], 1, Alignment::Right),
                TableCell::new_with_alignment(board[6], 1, Alignment::Right)
            ]),
            Row::new(vec![
                TableCell::new_with_alignment(board[7], 1, Alignment::Left),
                TableCell::new_with_alignment(board[8], 1, Alignment::Left),
                TableCell::new_with_alignment(board[9], 1, Alignment::Left),
                TableCell::new_with_alignment(board[10], 1, Alignment::Center),
                TableCell::new_with_alignment(board[11], 1, Alignment::Right),
                TableCell::new_with_alignment(board[12], 1, Alignment::Right),
                TableCell::new_with_alignment(board[13], 1, Alignment::Right)
            ]),
            Row::new(vec![
                TableCell::new_with_alignment(board[14], 1, Alignment::Left),
                TableCell::new_with_alignment(board[15], 1, Alignment::Left),
                TableCell::new_with_alignment(board[16], 1, Alignment::Left),
                TableCell::new_with_alignment(board[17], 1, Alignment::Center),
                TableCell::new_with_alignment(board[18], 1, Alignment::Right),
                TableCell::new_with_alignment(board[19], 1, Alignment::Right),
                TableCell::new_with_alignment(board[20], 1, Alignment::Right)
            ]),
            Row::new(vec![
                TableCell::new_with_alignment(board[21], 1, Alignment::Left),
                TableCell::new_with_alignment(board[22], 1, Alignment::Left),
                TableCell::new_with_alignment(board[23], 1, Alignment::Left),
                TableCell::new_with_alignment(board[24], 1, Alignment::Center),
                TableCell::new_with_alignment(board[25], 1, Alignment::Right),
                TableCell::new_with_alignment(board[26], 1, Alignment::Right),
                TableCell::new_with_alignment(board[27], 1, Alignment::Right)
            ]),
            Row::new(vec![
                TableCell::new_with_alignment(board[28], 1, Alignment::Left),
                TableCell::new_with_alignment(board[29], 1, Alignment::Left),
                TableCell::new_with_alignment(board[30], 1, Alignment::Left),
                TableCell::new_with_alignment(board[31], 1, Alignment::Center),
                TableCell::new_with_alignment(board[32], 1, Alignment::Right),
                TableCell::new_with_alignment(board[33], 1, Alignment::Right),
                TableCell::new_with_alignment(board[34], 1, Alignment::Right)
            ]),
            Row::new(vec![
                TableCell::new_with_alignment(board[35], 1, Alignment::Left),
                TableCell::new_with_alignment(board[36], 1, Alignment::Left),
                TableCell::new_with_alignment(board[37], 1, Alignment::Left),
                TableCell::new_with_alignment(board[38], 1, Alignment::Center),
                TableCell::new_with_alignment(board[39], 1, Alignment::Right),
                TableCell::new_with_alignment(board[40], 1, Alignment::Right),
                TableCell::new_with_alignment(board[41], 1, Alignment::Right)
            ])
        ]
    ).build();
    table.to_string();
    println!("\n{}\n", table.render());
}

fn is_game_over(board: &Vec<char>) -> bool {
    // Check for horizontal wins
    for row in 0..6 {
        for col in 0..4 {
            let idx = row * 7 + col;
            if board[idx] != ' ' && board[idx] == board[idx+1] && board[idx] == board[idx+2] && board[idx] == board[idx+3] {
                return true;
            }
        }
    }

    // Check for vertical wins
    for row in 0..3 {
        for col in 0..7 {
            let idx = row * 7 + col;
            if board[idx] != ' ' && board[idx] == board[idx+7] && board[idx] == board[idx+14] && board[idx] == board[idx+21] {
                return true;
            }
        }
    }

    // Check for diagonal wins (bottom left to top right)
    for row in 0..3 {
        for col in 0..4 {
            let idx = row * 7 + col;
            if board[idx] != ' ' && board[idx] == board[idx+8] && board[idx] == board[idx+16] && board[idx] == board[idx+24] {
                return true;
            }
        }
    }

    // Check for diagonal wins (top left to bottom right)
    for row in 3..6 {
        for col in 0..4 {
            let idx = row * 7 + col;
            if board[idx] != ' ' && board[idx] == board[idx-6] && board[idx] == board[idx-12] && board[idx] == board[idx-18] {
                return true;
            }
        }
    }

    // Check if the board is full
    for i in 0..board.len() {
        if board[i] == ' ' {
            return false;
        }
    }

    // If we got here, there are no wins and the board is full
    true
}

fn top_layout() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Connect  Four").unwrap();
    println!("{}", figure.to_string().blue());
}

pub fn main_game(){
    let mut board_vec: Vec<char> = vec![' '; 42];
    draw_table_board(&mut board_vec);
}