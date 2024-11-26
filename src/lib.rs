use rand::Rng;
use std::collections::HashMap;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io;

/// Rolls a dice to generate a random number between 1 and 6.
///
/// # Returns
/// A random number between 1 and 6 inclusive.
pub fn roll_dice() -> u32 {
    rand::thread_rng().gen_range(1..=6)
}

/// Displays the current state of the board, including player positions, snakes, and ladders.
///
/// # Parameters
/// - `player_positions`: A vector containing the positions of all players.
/// - `board_width`: The number of squares in each row of the board.
/// - `board_size`: The total number of squares on the board.
/// - `snakes`: A map of snake heads (keys) and tails (values).
/// - `ladders`: A map of ladder bottoms (keys) and tops (values).
pub fn display_board(
    player_positions: &Vec<u32>,
    board_width: usize,
    board_size: u32,
    snakes: &HashMap<u32, u32>,
    ladders: &HashMap<u32, u32>,
) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    println!("\nCurrent Board:");
    for row in (0..10).rev() {
        for col in 0..10 {
            let position = calculate_1d_position(row, col, board_width);

            // Determine the content of the cell
            if let Some(player_index) = player_positions
                .iter()
                .position(|&pos| pos == position as u32)
            {
                // Player cell (use different colors for each player).
                let player_number = player_index + 1;
                let player_color = if player_number%2 == 1 {
                    Color::Blue
                } else {
                    Color::Green
                };

                stdout
                    .set_color(ColorSpec::new().set_fg(Some(player_color)))
                    .unwrap();
                print!("P{}  ", player_number);
            } else if let Some(&end) = snakes.get(&(position as u32)) {
                // Snake cell (red).
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                    .unwrap();
                print!("S{}  ", end);
            } else if let Some(&end) = ladders.get(&(position as u32)) {
                // Ladder cell (yellow).
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
                    .unwrap();
                print!("L{}  ", end);
            } else {
                // Empty cell.
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::White)))
                    .unwrap();
                print!("{:2}  ", position);
            }
        }
        println!(); // Newline for the next row.
    }
    stdout.reset().unwrap(); // Reset color to default.
    println!(); // Extra spacing.
}

/// Calculates the 1D position of a square on the board based on its 2D coordinates.
///
/// # Parameters
/// - `row`: The row number (0-indexed).
/// - `col`: The column number (0-indexed).
/// - `board_width`: The number of squares in each row.
///
/// # Returns
/// The 1D position of the square on the board.
fn calculate_1d_position(row: usize, col: usize, board_width: usize) -> usize {
    let base = row * board_width;
    if row % 2 == 0 {
        base + col + 1 // Left-to-right for even rows.
    } else {
        base + (board_width - col) // Right-to-left for odd rows.
    }
}

/// Runs the game loop, allowing players to take turns and interact with the board.
///
/// # Parameters
/// - `num_players`: The number of players (between 2 and 4).
/// - `board_size`: The total number of squares on the board.
/// - `board_width`: The number of squares in each row of the board.
/// - `snakes`: A map of snake heads (keys) and tails (values).
/// - `ladders`: A map of ladder bottoms (keys) and tops (values).
pub fn start_game(
    num_players: usize,
    board_size: u32,
    board_width: usize,
    snakes: &HashMap<u32, u32>,
    ladders: &HashMap<u32, u32>,
) {
    let mut player_positions = vec![0; num_players]; // Initialize player positions
    let mut current_player = 0;
    let mut input = String::new();

    println!("Starting the Snakes and Ladders game with {} players!", num_players);

    // Game loop
    loop {
        println!(
            "Player {}'s turn. (Position: {})",
            current_player + 1,
            player_positions[current_player]
        );
        io::stdin().read_line(&mut input).unwrap();

        // Roll the dice and calculate the new position.
        let dice_roll = roll_dice();
        println!("You rolled a {}", dice_roll);

        let mut new_position = player_positions[current_player] + dice_roll;
        if new_position > board_size {
            new_position = player_positions[current_player]; // Stay in place if the roll exceeds the board size.
        }

        // Check for snakes or ladders at the new position.
        if let Some(&end) = snakes.get(&new_position) {
            println!("Oh no! You hit a snake! Sliding down to {}", end);
            new_position = end;
        } else if let Some(&end) = ladders.get(&new_position) {
            println!("Great! You found a ladder! Climbing up to {}", end);
            new_position = end;
        }

        player_positions[current_player] = new_position;
        println!(
            "Player {} is now at position {}",
            current_player + 1,
            new_position
        );

        display_board(&player_positions, board_width, board_size, snakes, ladders);

        if new_position == board_size {
            println!("Player {} wins!", current_player + 1);
            break;
        }

        current_player = (current_player + 1) % player_positions.len();
    }
}
