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
    println!("\nChoose the level of the bot (0-10): ");
    io::stdin()
        .read_line(&mut input_text_lvl)
        .expect("Failed to read from stdin");

    let level: usize = input_text_lvl.trim().parse::<usize>().unwrap_or_else(|_| {
        println!("This is not a number.");
        exit(0);
    });

    if level > 10 {
        println!("Only levels 0–10 are allowed.");
        exit(0);
    }

    let mut board_vec: Vec<char> = vec![' '; 63]; // 9 columns × 7 rows
    draw_table_board(&mut board_vec);

    loop {
        let mut input_text = String::new();
        println!("\nType the slot number you want to play (1–9): ");
        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read from stdin");

        let slot: usize = match input_text.trim().parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                println!("This is not a number.");
                continue;
            }
        };

        if (1..=9).contains(&slot) && is_available(&mut board_vec, slot as i32) {
            // Player move
            fill(&mut board_vec, slot, 'X');
            draw_table_board(&mut board_vec);
            println!("Actual Board Score: {}", evaluate_board(&mut board_vec));

            // Check for draw or win
            if is_game_over(&mut board_vec) {
                if possible_move(&mut board_vec).is_empty() {
                    let fig = FIGfont::standard().unwrap();
                    let msg = fig.convert("It's a Draw!").unwrap();
                    println!("\n{}", msg.to_string().magenta());
                } else {
                    let fig = FIGfont::standard().unwrap();
                    let msg = fig.convert("X Wins").unwrap();
                    println!("\n{}", msg.to_string().green());
                }
                break;
            }

            // Bot move
            let start_time = Instant::now();
            let bot = par_minimax(&mut board_vec, level as i32, false);
            let best_move = bot.0.unwrap();
            let score = bot.1;

            fill(&mut board_vec, best_move, 'O');
            let elapsed = start_time.elapsed();

            draw_table_board(&mut board_vec);
            println!("Bot drops slot number: {}", best_move);
            println!("Time Taken: {}.{} seconds", elapsed.as_secs(), elapsed.subsec_millis());
            println!("Board Score: {}", score);
            println!("Actual Board Score: {}", evaluate_board(&mut board_vec));

            // Check for draw or win
            if is_game_over(&mut board_vec) {
                if possible_move(&mut board_vec).is_empty() {
                    let fig = FIGfont::standard().unwrap();
                    let msg = fig.convert("It's a Draw!").unwrap();
                    println!("\n{}", msg.to_string().magenta());
                } else {
                    let fig = FIGfont::standard().unwrap();
                    let msg = fig.convert("You Suck").unwrap();
                    println!("\n{}", msg.to_string().red());
                }
                break;
            }
        } else {
            if !(1..=9).contains(&slot) {
                println!("Slot number {} is out of range. Please choose 1–9.", slot);
            } else {
                println!("Slot number {} is already full.", slot);
            }
        }
    }
}

pub fn draw_table_board(board: &mut Vec<char>) {
    let table = TableBuilder::new().style(TableStyle::extended()).rows(
        vec![
            // Header row
            Row::new(vec![
                TableCell::new_with_alignment("1", 1, Alignment::Center),
                TableCell::new_with_alignment("2", 1, Alignment::Center),
                TableCell::new_with_alignment("3", 1, Alignment::Center),
                TableCell::new_with_alignment("4", 1, Alignment::Center),
                TableCell::new_with_alignment("5", 1, Alignment::Center),
                TableCell::new_with_alignment("6", 1, Alignment::Center),
                TableCell::new_with_alignment("7", 1, Alignment::Center),
                TableCell::new_with_alignment("8", 1, Alignment::Center),
                TableCell::new_with_alignment("9", 1, Alignment::Center),
            ]),

            // Row 0
            Row::new(vec![
                TableCell::new_with_alignment(board[0], 1, Alignment::Left),
                TableCell::new_with_alignment(board[1], 1, Alignment::Left),
                TableCell::new_with_alignment(board[2], 1, Alignment::Left),
                TableCell::new_with_alignment(board[3], 1, Alignment::Center),
                TableCell::new_with_alignment(board[4], 1, Alignment::Center),
                TableCell::new_with_alignment(board[5], 1, Alignment::Center),
                TableCell::new_with_alignment(board[6], 1, Alignment::Right),
                TableCell::new_with_alignment(board[7], 1, Alignment::Right),
                TableCell::new_with_alignment(board[8], 1, Alignment::Right),
            ]),

            // Row 1
            Row::new(vec![
                TableCell::new_with_alignment(board[9], 1, Alignment::Left),
                TableCell::new_with_alignment(board[10], 1, Alignment::Left),
                TableCell::new_with_alignment(board[11], 1, Alignment::Left),
                TableCell::new_with_alignment(board[12], 1, Alignment::Center),
                TableCell::new_with_alignment(board[13], 1, Alignment::Center),
                TableCell::new_with_alignment(board[14], 1, Alignment::Center),
                TableCell::new_with_alignment(board[15], 1, Alignment::Right),
                TableCell::new_with_alignment(board[16], 1, Alignment::Right),
                TableCell::new_with_alignment(board[17], 1, Alignment::Right),
            ]),

            // Row 2
            Row::new(vec![
                TableCell::new_with_alignment(board[18], 1, Alignment::Left),
                TableCell::new_with_alignment(board[19], 1, Alignment::Left),
                TableCell::new_with_alignment(board[20], 1, Alignment::Left),
                TableCell::new_with_alignment(board[21], 1, Alignment::Center),
                TableCell::new_with_alignment(board[22], 1, Alignment::Center),
                TableCell::new_with_alignment(board[23], 1, Alignment::Center),
                TableCell::new_with_alignment(board[24], 1, Alignment::Right),
                TableCell::new_with_alignment(board[25], 1, Alignment::Right),
                TableCell::new_with_alignment(board[26], 1, Alignment::Right),
            ]),

            // Row 3
            Row::new(vec![
                TableCell::new_with_alignment(board[27], 1, Alignment::Left),
                TableCell::new_with_alignment(board[28], 1, Alignment::Left),
                TableCell::new_with_alignment(board[29], 1, Alignment::Left),
                TableCell::new_with_alignment(board[30], 1, Alignment::Center),
                TableCell::new_with_alignment(board[31], 1, Alignment::Center),
                TableCell::new_with_alignment(board[32], 1, Alignment::Center),
                TableCell::new_with_alignment(board[33], 1, Alignment::Right),
                TableCell::new_with_alignment(board[34], 1, Alignment::Right),
                TableCell::new_with_alignment(board[35], 1, Alignment::Right),
            ]),

            // Row 4
            Row::new(vec![
                TableCell::new_with_alignment(board[36], 1, Alignment::Left),
                TableCell::new_with_alignment(board[37], 1, Alignment::Left),
                TableCell::new_with_alignment(board[38], 1, Alignment::Left),
                TableCell::new_with_alignment(board[39], 1, Alignment::Center),
                TableCell::new_with_alignment(board[40], 1, Alignment::Center),
                TableCell::new_with_alignment(board[41], 1, Alignment::Center),
                TableCell::new_with_alignment(board[42], 1, Alignment::Right),
                TableCell::new_with_alignment(board[43], 1, Alignment::Right),
                TableCell::new_with_alignment(board[44], 1, Alignment::Right),
            ]),

            // Row 5
            Row::new(vec![
                TableCell::new_with_alignment(board[45], 1, Alignment::Left),
                TableCell::new_with_alignment(board[46], 1, Alignment::Left),
                TableCell::new_with_alignment(board[47], 1, Alignment::Left),
                TableCell::new_with_alignment(board[48], 1, Alignment::Center),
                TableCell::new_with_alignment(board[49], 1, Alignment::Center),
                TableCell::new_with_alignment(board[50], 1, Alignment::Center),
                TableCell::new_with_alignment(board[51], 1, Alignment::Right),
                TableCell::new_with_alignment(board[52], 1, Alignment::Right),
                TableCell::new_with_alignment(board[53], 1, Alignment::Right),
            ]),

            // Row 6
            Row::new(vec![
                TableCell::new_with_alignment(board[54], 1, Alignment::Left),
                TableCell::new_with_alignment(board[55], 1, Alignment::Left),
                TableCell::new_with_alignment(board[56], 1, Alignment::Left),
                TableCell::new_with_alignment(board[57], 1, Alignment::Center),
                TableCell::new_with_alignment(board[58], 1, Alignment::Center),
                TableCell::new_with_alignment(board[59], 1, Alignment::Center),
                TableCell::new_with_alignment(board[60], 1, Alignment::Right),
                TableCell::new_with_alignment(board[61], 1, Alignment::Right),
                TableCell::new_with_alignment(board[62], 1, Alignment::Right),
            ]),
        ]
    ).build();

    println!("\n{}\n", table.render());
}

fn is_game_over(board: &Vec<char>) -> bool {
    let rows = 7;
    let cols = 9;

    // Horizontal win
    for row in 0..rows {
        for col in 0..=cols - 5 {
            let idx = row * cols + col;
            let piece = board[idx];
            if piece != ' ' &&
                piece == board[idx + 1] &&
                piece == board[idx + 2] &&
                piece == board[idx + 3] &&
                piece == board[idx + 4]
            {
                return true;
            }
        }
    }

    // Vertical win
    for row in 0..=rows - 5 {
        for col in 0..cols {
            let idx = row * cols + col;
            let piece = board[idx];
            if piece != ' ' &&
                piece == board[idx + cols] &&
                piece == board[idx + 2 * cols] &&
                piece == board[idx + 3 * cols] &&
                piece == board[idx + 4 * cols]
            {
                return true;
            }
        }
    }

    // Diagonal (bottom-left to top-right) win
    for row in 0..=rows - 5 {
        for col in 0..=cols - 5 {
            let idx = row * cols + col;
            let piece = board[idx];
            if piece != ' ' &&
                piece == board[idx + cols + 1] &&
                piece == board[idx + 2 * (cols + 1)] &&
                piece == board[idx + 3 * (cols + 1)] &&
                piece == board[idx + 4 * (cols + 1)]
            {
                return true;
            }
        }
    }

    // Diagonal (top-left to bottom-right) win
    for row in 0..=rows - 5 {
        for col in 4..cols {
            let idx = row * cols + col;
            let piece = board[idx];
            if piece != ' ' &&
                piece == board[idx + cols - 1] &&
                piece == board[idx + 2 * (cols - 1)] &&
                piece == board[idx + 3 * (cols - 1)] &&
                piece == board[idx + 4 * (cols - 1)]
            {
                return true;
            }
        }
    }

    // Check if the board is full (draw)
    if board.iter().all(|&c| c != ' ') {
        return true;
    }

    false
}

fn possible_move(board: &Vec<char>) -> Vec<usize> {
    let mut new_lst: Vec<usize> = Vec::new();
    for i in 1..=9 {
        if is_available(board, i) {
            new_lst.push(i as usize);
        }
    }
    new_lst
}

fn is_available(board: &Vec<char>, slot: i32) -> bool {
    let cols = 9;

    // Convert slot to 0-indexed column
    let col = (slot - 1) as usize;

    // Top of the column (row 0)
    board[col] == ' '
}

fn fill(board: &mut Vec<char>, slot: usize, side: char) {
    let cols = 9;
    let rows = 7;
    let col = slot - 1;

    for row in (0..rows).rev() { // from bottom (6) to top (0)
        let idx = row * cols + col;
        if board[idx] == ' ' {
            board[idx] = side;
            break;
        }
    }
}

fn remove(board: &mut Vec<char>, slot: usize) {
    let cols = 9;
    let rows = 7;
    let col = slot - 1;

    for row in 0..rows {
        let idx = row * cols + col;
        if board[idx] != ' ' {
            board[idx] = ' ';
            break;
        }
    }
}

fn evaluate_board(board: &Vec<char>) -> i32 {
    let mut score = 0;

    // Evaluate rows (7 rows, 9 columns — so cols 0..=4 are valid for 5-in-a-row)
    for i in 0..7 {
        for j in 0..5 {
            let index = i * 9 + j;
            let row: Vec<char> = board[index..(index + 5)].to_vec();
            score += evaluate_sequence(&row);
        }
    }

    // Evaluate columns (9 cols, 7 rows — so rows 0..=2 valid for vertical 5-in-a-row)
    for i in 0..3 {
        for j in 0..9 {
            let index = i * 9 + j;
            let column: Vec<char> = vec![
                board[index],
                board[index + 9],
                board[index + 18],
                board[index + 27],
                board[index + 36],
            ];
            score += evaluate_sequence(&column);
        }
    }

    // Evaluate diagonals
    for i in 0..3 {
        for j in 0..5 {
            let index = i * 9 + j;
            let diagonal1: Vec<char> = vec![
                board[index],
                board[index + 10], // +9 (row) +1 (col)
                board[index + 20],
                board[index + 30],
                board[index + 40],
            ];
            score += evaluate_sequence(&diagonal1);

            let diagonal2: Vec<char> = vec![
                board[index + 4],       // start from offset in row
                board[index + 12],      // +9 (row) -1 (col)
                board[index + 20],
                board[index + 28],
                board[index + 36],
            ];
            score += evaluate_sequence(&diagonal2);
        }
    }

    score
}

fn evaluate_sequence(sequence: &Vec<char>) -> i32 {
    let mut score = 0;

    let player_count = sequence.iter().filter(|&c| *c == 'X').count();
    let opponent_count = sequence.iter().filter(|&c| *c == 'O').count();
    let empty_count = sequence.iter().filter(|&c| *c == ' ').count();

    if player_count == 5 {
        score += 10000;
    } else if player_count == 4 && empty_count == 1 {
        score += 100;
    } else if player_count == 3 && empty_count == 2 {
        score += 10;
    } else if player_count == 2 && empty_count == 3 {
        score += 2;
    }

    if opponent_count == 5 {
        score -= 10000;
    } else if opponent_count == 4 && empty_count == 1 {
        score -= 120;
    } else if opponent_count == 3 && empty_count == 2 {
        score -= 10;
    } else if opponent_count == 2 && empty_count == 3 {
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
            remove(board, col as i32 as usize);
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
            remove(board, col as i32 as usize);
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