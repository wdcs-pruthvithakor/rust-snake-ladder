# Snakes and Ladders

A graphical implementation of the classic **Snakes and Ladders** game written in **Rust**. Play with 2-4 players and enjoy the excitement of rolling dice, climbing ladders, and avoiding snakes as you race to the finish!

This game uses the **eframe** and **egui** libraries for a graphical user interface (GUI) and is available as a crate on [crates.io](https://crates.io/crates/snake_ladder).

---

## Features

- 🎲 **2-4 player gameplay**: Supports up to 4 players, taking turns to roll the dice.
- 🐍 **Snakes and Ladders mechanics**: Roll the dice, land on snakes to slide back, or climb ladders to advance.
- 🏆 **Winning Condition**: The first player to reach the final square (100) wins the game.
- 🎮 **Interactive UI**: A colorful, grid-based visual game board with player positions, snakes, and ladders.
- 📜 **Player status messages**: Get real-time updates about your dice roll, movement, and interactions with snakes and ladders.
- 🔄 **Player reset**: If a player lands on the same position as another, the other player is sent back to the start.

---

## Installation

### Install the binary

To install the Snakes and Ladders game globally as a command-line binary, run:

```bash
cargo install snake_ladder
```

Once installed, you can run the game directly from the terminal.

### Install as a library

To use the Snakes and Ladders game as a library in your own project, run the following command in your project directory:

```bash
cargo add snake_ladder
```

Or, manually add it to your `Cargo.toml` file:

```toml
[dependencies]
snake_ladder = "1.0.0"
```

---

## Running the Game

After installing the binary with `cargo install`, you can run the game using:

```bash
snake_ladder
```

This will launch the Snakes and Ladders game with a graphical user interface (GUI).

---

## Customization

You can customize the game by modifying the **board size** and the positions of **snakes** and **ladders**. 

### Example of Customizing Snakes and Ladders

You can modify the `snakes` and `ladders` positions by passing new values in the `main.rs` file where the game is initialized. Here's an example:

```rust
use snake_ladder::Game;
use std::collections::HashMap;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    
    // Custom Snakes and Ladders positions
    let snakes = HashMap::from([
        (16, 6),  // Snake from 16 to 6
        (47, 26), // Snake from 47 to 26
        (49, 11), // Snake from 49 to 11
        (56, 53), // Snake from 56 to 53
        (62, 19), // Snake from 62 to 19
        (64, 60), // Snake from 64 to 60
        (87, 24), // Snake from 87 to 24
        (93, 73), // Snake from 93 to 73
        (95, 75), // Snake from 95 to 75
        (98, 78), // Snake from 98 to 78
    ]);

    let ladders = HashMap::from([
        (1, 38),  // Ladder from 1 to 38
        (4, 14),  // Ladder from 4 to 14
        (9, 31),  // Ladder from 9 to 31
        (21, 42), // Ladder from 21 to 42
        (28, 84), // Ladder from 28 to 84
        (36, 44), // Ladder from 36 to 44
        (51, 67), // Ladder from 51 to 67
        (71, 91), // Ladder from 71 to 91
        (80, 100), // Ladder from 80 to 100
    ]);
    
    // Running the game with the custom snakes and ladders
    eframe::run_native(
        "Snakes and Ladders",
        options,
        Box::new(|_cc| Ok(Box::new(Game::new(snakes, ladders)))),
    )
}
```

In the above code:

- **Snakes**: If a player lands on a snake's head (e.g., 16), they will move to its tail (e.g., 6).
- **Ladders**: If a player lands at the bottom of a ladder (e.g., 1), they will climb up to the top (e.g., 38).

