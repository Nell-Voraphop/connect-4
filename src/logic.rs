use term_table::table_cell::{Alignment, TableCell};
use term_table::{TableBuilder, TableStyle};
use term_table::row::Row;
use rayon::prelude::*;
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

fn possible_move(board: &mut Vec<char>) -> Vec<usize> {
    let mut new_lst: Vec<usize> = Vec::new();
    for i in 1..8 {
        if is_available(board, i) {
            new_lst.push(i as usize);
        }
    }
    new_lst
}

fn is_available(board: &mut Vec<char>, slot: i32) -> bool{
    let mut state: bool = true;
    for i in 1..7 {
        if board[(-7*i + 41 + slot) as usize] != ' ' && i != 6{
            continue
        }
        else if board[(-7*i + 41 + slot) as usize] != ' ' && i == 6{
            state = false
        }

    }
    state
}

fn fill(board: &mut Vec<char>, slot: usize, side: char) {
    for i in 1..7 {
        let temp_slot :i32 = slot.wrapping_sub(1) as i32;
        if board[(-7*i + 42 + temp_slot) as usize] != ' ' {
            continue
        }
        else {
            board[(-7*i + 42 + temp_slot) as usize] = side;
            break
        }
    }
}

fn remove(board: &mut Vec<char>, slot: i32) {
    for i in 1..7 {
        if board[(7*i - 8 + slot) as usize] == ' ' {
            continue
        }
        else {
            board[(7*i - 8 + slot) as usize] = ' ';
            break
        }
    }
}

fn evaluate_board(board: &mut Vec<char>) -> i32 {
    let mut score = 0;

    // Evaluate rows
    for i in 0..6 {
        for j in 0..4 {
            let index = i * 7 + j;
            let row: Vec<char> = board[index..(index + 4)].to_vec();
            score += evaluate_sequence(&row);
        }
    }

    // Evaluate columns
    for i in 0..3 {
        for j in 0..7 {
            let index = i * 7 + j;
            let column: Vec<char> = vec![
                board[index],
                board[index + 7],
                board[index + 14],
                board[index + 21],
            ];
            score += evaluate_sequence(&column);
        }
    }

    // Evaluate diagonals
    for i in 0..3 {
        for j in 0..4 {
            let index = i * 7 + j;
            let diagonal1: Vec<char> = vec![
                board[index],
                board[index + 8],
                board[index + 16],
                board[index + 24],
            ];
            score += evaluate_sequence(&diagonal1);

            let diagonal2: Vec<char> = vec![
                board[index + 3],
                board[index + 9],
                board[index + 15],
                board[index + 21],
            ];
            score += evaluate_sequence(&diagonal2);
        }
    }
    score
}

fn evaluate_sequence(sequence: &Vec<char>) -> i32 {
    let mut score = 0;
    // Evaluate the number of player pieces and empty spaces in the sequence
    let player_count = sequence.iter().filter(|&c| *c == 'X').count();
    let opponent_count = sequence.iter().filter(|&c| *c == 'O').count();
    let empty_count = sequence.iter().filter(|&c| *c == ' ').count();

    // Assign a score based on the number of player pieces and empty spaces
    if player_count == 4 {
        score += 1000;
    } else if player_count == 3 && empty_count == 1 {
        score += 5;
    } else if player_count == 2 && empty_count == 2 {
        score += 2;
    }

    // Subtract a score based on the number of opponent pieces in the sequence
    if opponent_count == 4 {
        score -= 1000;
    }
    if opponent_count == 3 && empty_count == 1 {
        score -= 5;
    } else if opponent_count == 2 && empty_count == 2 {
        score -= 2;
    }
    score
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