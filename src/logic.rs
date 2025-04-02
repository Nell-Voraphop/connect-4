use term_table::table_cell::{Alignment, TableCell};
use term_table::{TableBuilder, TableStyle};
use rand::prelude::*;
use term_table::row::Row;
use std::process::exit;
use std::time::Instant;
use rayon::prelude::*;
use figlet_rs::*;
use colored::*;
use std::io;

fn logic() {
    let mut input_text_lvl = String::new();
    println!("\nChoose the level of the bot (0-8) \r");
    io::stdin()
        .read_line(&mut input_text_lvl)
        .expect("failed to read from stdin");

    let level: usize = input_text_lvl.trim().parse::<usize>().unwrap_or_else(|_| {
        println!("This is not a number");
        exit(0)
    });
    if level > 9 || level < 0 {
        println!("There are only 0-8 levels");
        exit(0)
    }

    let mut board_vec: Vec<char> = vec![' '; 42];
    draw_table_board(&mut board_vec);

    loop {
        let mut input_text = String::new();
        println!("\nType in the slot number you want to play (1-7): \r");
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let slot: usize = match input_text.trim().parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                println!("This is not a number");
                break;
            }
        };

        if slot.to_string().trim().parse::<usize>().is_ok() {
            if slot <= 7 && slot > 0 && is_available(&mut board_vec, slot as i32) {
                fill(&mut board_vec, slot, 'X');
                draw_table_board(&mut board_vec);
                println!("Actual Board Score = { }", evaluate_board(&mut board_vec));
                if is_game_over(&mut board_vec) && possible_move(&mut board_vec).is_empty(){
                    let standard_font = FIGfont::standard().unwrap();
                    let drawer = standard_font.convert("It\'s a Draw!").unwrap();
                    println!("\n{}", drawer.to_string().magenta());
                    break;
                }
                else if is_game_over(&mut board_vec) && !possible_move(&mut board_vec).is_empty(){
                    let standard_font = FIGfont::standard().unwrap();

                    let x_winner = standard_font.convert("X Wins").unwrap();
                    println!("\n{}", x_winner.to_string().green());
                    break;
                }
                let start_time = Instant::now();

                let bot = par_minimax(&mut board_vec, level as i32, false);
                //###################################################################################################################

                let best_move = bot.0;
                let score = bot.1;
                fill(&mut board_vec, best_move.unwrap(), 'O');
                let end_time = start_time.elapsed();
                let seconds = end_time.as_secs();
                let millis = end_time.subsec_millis();
                draw_table_board(&mut board_vec);
                println!("Bot drops slot number : {:?}", best_move.unwrap());
                println!("Time Taken: {}.{} seconds", seconds, millis);
                println!("Board Score = { }", score);
                println!("Actual Board Score = { }", evaluate_board(&mut board_vec));
                if is_game_over(&mut board_vec) && possible_move(&mut board_vec).is_empty(){
                    let standard_font = FIGfont::standard().unwrap();
                    let drawer = standard_font.convert("It\'s a Draw!").unwrap();
                    println!("\n{}", drawer.to_string().magenta());
                    break;
                }
                else if is_game_over(&mut board_vec) && !possible_move(&mut board_vec).is_empty(){
                    let standard_font = FIGfont::standard().unwrap();

                    let x_winner = standard_font.convert("You Suck").unwrap();
                    println!("\n{}", x_winner.to_string().green());
                    break;
                }
            } else {
                if slot <= 7 && slot > 0 {
                    println!("Number out of range")
                }
                else {
                    println!("Slot number {} is already full", slot)
                }
            }
        } else {
            println!("This is not a number.");
        }
    }
}

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

fn possible_move(board: &Vec<char>) -> Vec<usize> {
    let mut new_lst: Vec<usize> = Vec::new();
    for i in 1..8 {
        if is_available(board, i) {
            new_lst.push(i as usize);
        }
    }
    new_lst
}

fn is_available(board: &Vec<char>, slot: i32) -> bool{
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

fn evaluate_board(board: &Vec<char>) -> i32 {
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

fn par_minimax(board: &Vec<char>, depth: i32, maximizing_player: bool) -> (Option<usize>, i32) {
    if depth == 0 || is_game_over(board) {
        return (None, evaluate_board(board));
    };

    let moves = possible_move(board);
    let mut rng = rand::thread_rng();
    let default_move = *moves.choose(&mut rng).unwrap();

    let results: Vec<(usize, i32)> = moves
        .par_iter()
        .map(|&col| {
            let mut copy_board = board.clone();
            let player = if maximizing_player { 'X' } else { 'O' };
            fill(&mut copy_board, col, player);
            let score = alpha_beta(&mut copy_board, depth - 1, i32::MIN, i32::MAX, !maximizing_player);
            (col, score)
        })
        .collect();

    let best = if maximizing_player {
        results.into_iter().max_by_key(|&(_, score)| score).unwrap_or((default_move, 0))
    } else {
        results.into_iter().min_by_key(|&(_, score)| score).unwrap_or((default_move, 0))
    };

    (Some(best.0), best.1)
}

fn alpha_beta(board: &mut Vec<char>, depth: i32, mut alpha: i32, mut beta: i32, maximizing: bool) -> i32 {
    if depth == 0 || is_game_over(board) {
        return evaluate_board(board);
    }

    let moves = possible_move(board);

    if maximizing {
        let mut max_eval = i32::MIN;
        for &col in &moves {
            fill(board, col, 'X');
            let eval = alpha_beta(board, depth - 1, alpha, beta, false);
            remove(board, col as i32);
            max_eval = max_eval.max(eval);
            alpha = alpha.max(eval);
            if beta <= alpha {
                break; // Beta cut-off
            }
        }
        max_eval
    } else {
        let mut min_eval = i32::MAX;
        for &col in &moves {
            fill(board, col, 'O');
            let eval = alpha_beta(board, depth - 1, alpha, beta, true);
            remove(board, col as i32);
            min_eval = min_eval.min(eval);
            beta = beta.min(eval);
            if beta <= alpha {
                break; // Alpha cut-off
            }
        }
        min_eval
    }
}

pub fn main_game() {
    top_layout();
    println!("\nWelcome to Connect 4 Game! Your goal is to connect 4 red in order to win the game.\n\
                You can only type in the number in range of 1 to 7, the number you type in will drop the red piece into the column of your input. \n\
                The board is already provided you the number of column. Have fun ^^");

    loop {
        logic();
        println!("\n\nContinue playing? (y/n): ");
        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Invalid input!");
        if response.contains(&"n") {
            println!("{}", "Exiting program ...".to_string().red());
            exit(0);
        }
        else {
            continue;
        }
    }
}