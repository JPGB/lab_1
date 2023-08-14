use std::io;

fn main() {
    let mut player = 'O';

    let mut board: Vec<char> = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    loop {
        draw_board(&board);

        if player == 'O' {
            player = 'X'
        } else {
            player = 'O'
        }

        println!("\nPlayer {player}, choose a tile from the board");

        let mut user_input = String::new();

        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                if !validate_player_choice(&user_input, &mut board, player.clone()) {
                    if player == 'O' {
                        player = 'X'
                    } else {
                        player = 'O'
                    }
                    if !check_board_complete(&board) {
                        draw_board(&board);

                        println!("It's a draw!");
                        break;
                    }
                } else {
                    if check_if_won(&board) {
                        draw_board(&board);

                        println!("Player {}, won!", player);

                        break;
                    }
                    if !check_board_complete(&board) {
                        draw_board(&board);

                        println!("It's a draw!");
                        break;
                    }
                }
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
}

fn draw_board(board: &Vec<char>) {
    println!(" -------------");
    for index in 0..9 {
        print!(" | {}", board[index]);
        if (index + 1) % 3 == 0 {
            print!(" |\n");
            println!(" -------------");
        }
    }
}

fn validate_player_choice(user_input: &String, board: &mut Vec<char>, player: char) -> bool {
    match user_input.trim().parse::<usize>() {
        Ok(mut chosen_index) => {
            chosen_index -= 1;
            if board[chosen_index] != 'X' && board[chosen_index] != 'O' {
                board[chosen_index] = player;
                return true;
            } else {
                println!("\nAlready chosen!");
                return false;
            }
        }
        Err(error) => {
            println!("Error: {}", error);
            return false;
        }
    }
}

fn check_if_won(board: &Vec<char>) -> bool {
    if check_lines(&board) || check_column(&board) || check_diagonals(&board) {
        return true;
    } else {
        return false;
    }
}

fn check_lines(board: &Vec<char>) -> bool {
    let mut verifier: Vec<char> = vec![];

    for index in 0..9 {
        verifier.push(board[index]);
        if verifier.len() == 3 {
            verifier.dedup();
            if verifier.len() == 1 {
                return true;
            } else {
                verifier = vec![];
            }
        }
    }

    return false;
}

fn check_column(board: &Vec<char>) -> bool {
    let mut verifier: Vec<char> = vec![];

    for index in 0..3 {
        verifier.push(board[index]);
        for counter in 0..2 {
            let index_col = index + (3 * (counter + 1));
            verifier.push(board[index_col]);
            if verifier.len() == 3 {
                verifier.dedup();
                if verifier.len() == 1 {
                    return true;
                } else {
                    verifier = vec![];
                }
            }
        }
    }

    return false;
}

fn check_diagonals(board: &Vec<char>) -> bool {
    let mut verifier: Vec<char> = vec![];

    let mut index = 0;
    verifier.push(board[index]);

    for counter in 0..2 {
        let index_col = index + (4 * (counter + 1));
        verifier.push(board[index_col]);
        if verifier.len() == 3 {
            verifier.dedup();
            if verifier.len() == 1 {
                return true;
            } else {
                verifier = vec![];
            }
        }
    }

    verifier = vec![];

    index = 2;
    verifier.push(board[index]);

    for counter in 0..2 {
        let index_col = index + (2 * (counter + 1));
        verifier.push(board[index_col]);
        if verifier.len() == 3 {
            verifier.dedup();
            if verifier.len() == 1 {
                return true;
            } else {
                verifier = vec![];
            }
        }
    }

    return false;
}

fn check_board_complete(board: &Vec<char>) -> bool {
    let mut has_space_in_board = false;

    for index in 0..9 {
        if board[index] != 'X' && board[index] != 'O' {
            has_space_in_board = true;
        }
    }

    has_space_in_board
}
