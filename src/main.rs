struct Board {
    pub spaces: [char; 9],
}

impl Board {
    fn new() -> Self {
        Board { spaces: [' '; 9] }
    }

    fn print(&self) {
        println!("{}|{}|{}", self.spaces[0], self.spaces[1], self.spaces[2]);
        println!("-----");
        println!("{}|{}|{}", self.spaces[3], self.spaces[4], self.spaces[5]);
        println!("-----");
        println!("{}|{}|{}", self.spaces[6], self.spaces[7], self.spaces[8]);
    }

    fn check_winner(&self, player: &Player) -> bool {
        let player_char = Player::player_char(player);
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
            if self.spaces[combination[0]] == player_char
                && self.spaces[combination[1]] == player_char
                && self.spaces[combination[2]] == player_char
            {
                return true;
            }
        }
        false
    }

    fn cats(&self) -> bool {
        self.spaces.iter().all(|&c| c != ' ')
    }

    fn at(&self, index: usize) -> char {
        self.spaces[index]
    }

    fn set(&mut self, index: usize, value: &Player) {
        let player_char = Player::player_char(value);
        self.spaces[index] = player_char;
    }
}

enum Player {
    X,
    O,
}

impl Player {
    fn player_char(player: &Player) -> char {
        match player {
            Player::X => 'X',
            Player::O => 'O',
        }
    }
}

fn main() {
    let mut board = Board::new();

    let mut player = Player::X;
    loop {
        board.print();
        let mut input = String::new();
        println!("Enter a number between 1 and 9:");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
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
        if board.at(move_index) != ' ' {
            println!("Invalid move. Try again.");
            continue;
        }
        board.set(move_index, &player);
        if board.check_winner(&player) {
            board.print();
            println!("Player {} wins!", Player::player_char(&player));
            break;
        }
        if board.cats() {
            board.print();
            println!("It's a draw!");
            break;
        }
        println!("Player {}'s turn", Player::player_char(&player));
        player = match player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }
}
