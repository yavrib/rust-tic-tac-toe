use std::io;
use std::collections::HashMap;

struct Game {
    player1: HashMap<String, String>,
    player2: HashMap<String, String>,
    turn: String,
    board: Vec<String>,
    winner: String
}

fn main() {
    let mut game = Game { 
        player1: HashMap::new(),
        player2: HashMap::new(),
        turn: String::new(),
        board: vec![],
        winner: String::new()
    };
    let mut player1 = HashMap::new();
    let mut player2 = HashMap::new();
    let mut player1_name = String::new();
    let mut player2_name = String::new();
    let mut board: Vec<String> = vec![];

    for _index in 0..10 {
        board.push(String::from("None"));
    }

    game.board = board;

    println!("Board is ready!\nBoard: {:?}", game.board);

    println!("Please enter player 1 name", );
    io::stdin().read_line(&mut player1_name)
        .expect("Failed to read!");

    println!("Please enter player 2 name", );
    io::stdin().read_line(&mut player2_name)
        .expect("Failed to read!");

    player1.insert(String::from("Name"), player1_name.trim_matches('\n').to_string());
    player1.insert(String::from("Faction"), String::from("O"));
    player2.insert(String::from("Name"), player2_name.trim_matches('\n').to_string());
    player2.insert(String::from("Faction"), String::from("X"));

    game.player1 = player1;
    game.player2 = player2;

    match game.player1.get(&String::from("Name")) {
        Some(_) => game.turn = String::from("player1"),
        None => println!("{:?}", "Failed")
    }

    println!("{:?}", game.player1);
    println!("{:?}", game.player2);
    println!("{:?}", game.turn);
    println!("{:?}", game.board);
    println!("{:?}", game.winner);
    println!("{:?}", game.player1.get(&String::from("Name")).unwrap().to_string());

    loop {
        let mut player_move = String::new();

        println!("{:?}", game.board);
        println!("Please enter a position with the label \"None\".");

        io::stdin().read_line(&mut player_move)
            .expect("Failed to read!");

        let position: usize = player_move.trim().parse().unwrap();

        // Could not figure out a way to do this with pattern matching. Will fix later, hopefully.
        if game.turn == String::from("player1") && game.board[position] == String::from("None") {
            game.board[position] = game.player1.get(&String::from("Faction")).unwrap().to_string();
        } else if game.turn == String::from("player2") && game.board[position] == String::from("None") {
            game.board[position] = game.player2.get(&String::from("Faction")).unwrap().to_string();
        } else {
            println!("This part of the board is already captured! Please pick another field!");
            continue;
        };

        // Check if won and set winner
        
        // Could not figure out a way to do this with pattern matching. Will fix later, hopefully.
        if game.turn == String::from("player1") {
            game.turn = String::from("player2");
        } else if game.turn == String::from("player2") {
            game.turn = String::from("player1");
        } else {
            panic!("Could not find any players!");
        }

        // Could not figure out a way to do this with pattern matching. Will fix later, hopefully.
        if (game.winner == *game.player1.get(&String::from("Name")).unwrap()) || (game.winner == *game.player2.get(&String::from("Name")).unwrap()) {
            break;
        } else if game.winner == String::from("Tie") {
            break;
        }
    }
}
