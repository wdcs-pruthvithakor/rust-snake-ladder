use snake_ladder::Game;
use std::collections::HashMap;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
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
    ]);
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
    ]);
    eframe::run_native(
        "Snakes and Ladders",
        options,
        Box::new(|_cc| Ok(Box::new(Game::new(snakes, ladders)))),
    )
}
