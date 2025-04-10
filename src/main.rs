fn main() {
    let mut board: [char; 9] = [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
    let mut x_turn = true;
    loop {
        print_board(board);
        let mut input = String::new();
        println!("Enter a number between 1 and 9:");
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let move_index: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between 1 and 9.");
                continue;
            }
        };
        if move_index < 1 || move_index > 9 {
            println!("Invalid input. Please enter a number between 1 and 9.");
            continue;
        }
        let move_index = (move_index - 1) as usize;
        if board[move_index] != ' ' {
            println!("Invalid move. Try again.");
            continue;
        }
        board[move_index] = if x_turn { 'X' } else { 'O' };
        if check_winner(&board, if x_turn { 'X' } else { 'O' }) {
            print_board(board);
            println!("Player {} wins!", if x_turn { 'X' } else { 'O' });
            break;
        }
        if board.iter().all(|&c| c != ' ') {
            print_board(board);
            println!("It's a draw!");
            break;
        }
        if x_turn {
            println!("Player X's turn");
        } else {
            println!("Player O's turn");
        }
        x_turn = !x_turn;
    }
}

fn print_board(board: [char; 9]) {
    println!("{}|{}|{}", board[0], board[1], board[2]);
    println!("-----");
    println!("{}|{}|{}", board[3], board[4], board[5]);
    println!("-----");
    println!("{}|{}|{}", board[6], board[7], board[8]);
}

fn check_winner(board: &[char; 9], player: char) -> bool {
    let winning_combinations = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];
    for combination in winning_combinations.iter() {
        if board[combination[0]] == player
            && board[combination[1]] == player
            && board[combination[2]] == player
        {
            return true;
        }
    }
    false
}