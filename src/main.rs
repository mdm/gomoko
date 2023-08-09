fn main() {
    print_title();
    let mut board = vec![vec![0usize; 19]; 19];
    print_instructions();
    let board_size = get_board_size();

    loop {
        let next_move = ask_move(board_size);
        if board[next_move.1][next_move.0] != 0 {
            println!("SQUARE OCCUPIED.  TRY AGAIN...");
            continue;
        }
        board[next_move.1][next_move.0] = 1;
    }
}

fn make_intelligent_move(board_size: usize, player_move: (usize, usize)) -> Option<(usize, usize)> {
    for e in [-1, 0, 1] {
        for f in [-1, 0, 1] {
            if e + f - e * f == 0 {
                continue;
            }

            let x = player_move.0 + e;
            let y = player_move.1 + f;

            if !in_bounds(board_size, x, y) {
                continue;
            }
        }
    }
}

fn print_title() {
    println!("{: >39}", "GOMOKO");
    println!("{: >47}", "CREATIVE COMPUTING  MORRISTOWN, NEW JERSEY");
    println!("\n\n");
}

fn print_instructions() {
    println!("WELCOME TO THE ORIENTAL GAME OF GOMOKO.");
    println!("\nTHE GAME IS PLAYED ON AN N BY N GRID OF A SIZE");
    println!("THAT YOU SPECIFY.  DURING YOUR PLAY, YOU MAY COVER ONE GRID");
    println!("INTERSECTION WITH A MARKER. THE OBJECT OF THE GAME IS TO GET");
    println!("5 ADJACENT MARKERS IN A ROW -- HORIZONTALLY, VERTICALLY, OR");
    println!("DIAGONALLY.  ON THE BOARD DIAGRAM, YOUR MOVES ARE MARKED");
    println!("WITH A '1' AND THE COMPUTER MOVES WITH A '2'.");
    println!("\nTHE COMPUTER DOES NOT KEEP TRACK OF WHO HAS WON.");
    println!("TO END THE GAME, TYPE -1,-1 FOR YOUR MOVE.\n");
}

fn get_board_size() -> usize {
    println!("WHAT IS YOUR BOARD SIZE (MIN 7/ MAX 19)");

    let stdin = std::io::stdin();
    loop {
        let mut buffer = String::new();
        if stdin.read_line(&mut buffer).is_ok() {
            if let Ok(n) = buffer.trim().parse::<usize>() {
                if n > 6 && n < 20 {
                    return n;
                }
            }
        }

        println!("I SAID, THE MINIMUM IS 7, THE MAXIMUM IS 19.");
    }
}

fn ask_move(board_size: usize) -> (usize, usize) {
    println!("\nWE ALTERNATE MOVES.  YOU GO FIRST...\n");
    println!("YOUR PLAY (I,J)");

    let stdin = std::io::stdin();
    loop {
        let mut buffer = String::new();
        if stdin.read_line(&mut buffer).is_ok() {
            if let Some((x, y)) = buffer.split_once(',') {
                if let (Ok(x), Ok(y)) = (x.trim().parse::<usize>(), y.trim().parse::<usize>()) {
                    return in_bounds(board_size, x, y);
                }
            }
        }

        println!("ILLEGAL MOVE.  TRY AGAIN...");
    }
}

fn in_bounds(board_size: usize, x: usize, y: usize) -> bool {
    return x >= 1 && x <= board_size && y >= 1 && y <= board_size;
}
