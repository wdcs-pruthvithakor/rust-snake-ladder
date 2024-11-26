use std::collections::HashMap;
use std::io;
use snake_ladder::start_game; // Import from lib.rs

fn main() {
    let board_size = 100; // Total number of squares on the board.
    let board_width = 10; // Number of squares per row.
    let snakes = HashMap::from([
        (16, 6),
        (47, 26),
        (49, 11),
        (56, 53),
        (62, 19),
        (64, 60),
        (87, 24),
        (93, 73),
        (95, 75),
        (98, 78),
    ]); // Snake heads (keys) and tails (values).
    let ladders = HashMap::from([
        (1, 38),
        (4, 14),
        (9, 31),
        (21, 42),
        (28, 84),
        (36, 44),
        (51, 67),
        (71, 91),
        (80, 100),
    ]); // Ladder bottoms (keys) and tops (values).

    // Ask for the number of players (between 2 and 4).
    let mut num_players = String::new();
    println!("Enter the number of players (2-4): ");
    io::stdin().read_line(&mut num_players).unwrap();
    let num_players: usize = num_players.trim().parse().unwrap();

    if !(2..=4).contains(&num_players) {
        println!("Invalid number of players. Please choose between 2 and 4 players.");
        return;
    }

    // Start the game with the chosen number of players
    start_game(num_players, board_size, board_width, &snakes, &ladders);
}
