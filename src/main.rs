use std::io::{stdout, stdin, Write};

fn main() {
    println!("Choose a mode:");
    println!("1) Single player");
    println!("2) Two player");
    println!("3) Print help");
    stdout().flush().unwrap();
    let mut x: String = String::new();
    stdin().read_line(&mut x).unwrap();
    let y: u8 = x.trim().parse().unwrap();
    let mut board: [(u8, bool); 9] = [(0, false); 9];
    match y {
        1 => {
            println!("Beginning single player mode.");
            print!("Would you like to be Player 1 or Player 2? ");
            stdout().flush().unwrap();
            stdin().read_line(&mut x).unwrap();
            let player: u8 = x.trim().parse::<u8>().unwrap();
        },
        2 => {
            println!("Beginning two player mode.");
            let mut player: u8 = 1;
            loop {
                print_board(board);
                let mut z: usize;
                loop {
                    print!("Player {} turn: ", player);
                    stdout().flush().unwrap();
                    stdin().read_line(&mut x).unwrap();
                    z = x.trim().parse::<isize>().unwrap().try_into().unwrap();
                    if !(1..=9).contains(&z) {
                        eprintln!("Invalid spot: '{}'", z);
                    } else if board[z].1 {
                        println!("Player {} has already claimed this spot!", board[z].0);
                    } else { break };
                }
                board[z].1 = true;
                board[z].0 = player;
                player = if player == 1 { 2 } else { 1 }
            }
        },
        3 => {
            println!("Pick a number between 1 and 9, each corresponding to a slot in the grid.");
        }
        a => println!("Invalid option '{}'", a),
    }
}

fn print_board(board: [(u8, bool); 9]) {
    for i in 0..3 {
        println!("| {} | {} | {} |", x(board, i, 0), x(board, i, 1), x(board, i, 2));
    }
}

fn x(board: [(u8, bool); 9], i: usize, j: usize) -> String {
    if board[i * 3 + j].1 { match board[i * 3 + j].0 {
        1 => "o",
        2 => "x",
        a => panic!("Invalid player {}", a)
    }} else { " " }.to_owned()
}

