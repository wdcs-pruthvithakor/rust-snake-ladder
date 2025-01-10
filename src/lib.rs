use eframe::egui;
use rand::Rng;
use std::collections::HashMap;

#[derive(Default)]
pub struct Game {
    player_positions: Vec<u32>,
    current_player: usize,
    dice_roll: Option<u32>,
    snakes: HashMap<u32, u32>,
    ladders: HashMap<u32, u32>,
    winner: Option<usize>,
    num_players: usize,
    board_size: u32,
    status_message: String,
    game_started: bool,
}

impl Game {
    pub fn new(snakes: HashMap<u32, u32>, ladders: HashMap<u32, u32>) -> Self {
        Self {
            player_positions: vec![],
            current_player: 0,
            dice_roll: None,
            snakes,
            ladders,
            winner: None,
            num_players: 0,
            board_size: 100,
            status_message: String::new(),
            game_started: false,
        }
    }

    fn start_game(&mut self, num_players: usize) {
        self.num_players = num_players;
        self.player_positions = vec![0; num_players];
        self.current_player = 0;
        self.winner = None;
        self.dice_roll = None;
        self.status_message = String::from("Game started! Player 1's turn.");
        self.game_started = true;
    }

    fn roll_dice(&mut self) {
        if self.winner.is_none() {
            let roll = rand::thread_rng().gen_range(1..=6);
            self.dice_roll = Some(roll);
            self.status_message = format!("Player {} rolled a {}", self.current_player + 1, roll);
            self.update_position(); // Automatically move after rolling dice.
        }
    }

    fn update_position(&mut self) {
        if let Some(dice_roll) = self.dice_roll {
            if let Some(position) = self.player_positions.get_mut(self.current_player) {
                let mut new_position = *position + dice_roll;

                if new_position > self.board_size {
                    new_position = *position; // Stay in place if roll exceeds the board size.
                }

                if let Some(&snake_tail) = self.snakes.get(&new_position) {
                    self.status_message = format!(
                        "Player {} rolled {}, climbed to {}, but was bitten by a snake and fell to {}!",
                        self.current_player + 1,
                        dice_roll,
                        new_position,
                        snake_tail
                    );
                    new_position = snake_tail;
                } else if let Some(&ladder_top) = self.ladders.get(&new_position) {
                    self.status_message = format!(
                        "Player {} rolled {}, climbed a ladder from {} to {}!",
                        self.current_player + 1,
                        dice_roll,
                        new_position,
                        ladder_top
                    );
                    new_position = ladder_top;
                } else {
                    self.status_message = format!(
                        "Player {} rolled {}, and moved to position {}",
                        self.current_player + 1,
                        dice_roll,
                        new_position
                    );
                }
                *position = new_position;

                // After updating the current player's position, check if another player is already there
                let mut reset_player = None;
                for (other_player_index, &other_player_position) in
                    self.player_positions.iter().enumerate()
                {
                    if other_player_index != self.current_player
                        && other_player_position == new_position
                    {
                        // If another player is at the same position, mark them to be reset to 0
                        reset_player = Some(other_player_index);
                        self.status_message = format!(
                            "Oops! Player {} rolled {} and moved to position {} and sent Player {} back to home base!",
                            self.current_player + 1,
                            dice_roll,
                            new_position,
                            other_player_index + 1,
                        );
                        break;
                    }
                }

                // If another player was found at the same position, reset their position
                if let Some(player_to_reset) = reset_player {
                    self.player_positions[player_to_reset] = 0;
                }

                if new_position == self.board_size {
                    self.winner = Some(self.current_player);
                    self.status_message = format!("Player {} wins!", self.current_player + 1);
                } else {
                    self.current_player = (self.current_player + 1) % self.num_players;
                }

                self.dice_roll = None; // Reset dice roll.
            }
        }
    }

    fn render_board(&self, ui: &mut egui::Ui) {
        let board_width = 10;
        // let cell_width = 6; // Fixed width for each cell

        // Render the board grid
        for row in (0..board_width).rev() {
            ui.horizontal(|ui| {
                for col in 0..board_width {
                    let cell_number = if row % 2 == 0 {
                        row * board_width + col + 1
                    } else {
                        (row + 1) * board_width - col
                    } as u32;
                    let mut cell_width = 20 - format!("{}", cell_number).len();
                    let mut cell_label = format!("{:<cell_width$}", cell_number);
                    let mut cell_color = egui::Color32::WHITE;

                    // Check for players
                    for (player_index, &player_position) in self.player_positions.iter().enumerate()
                    {
                        if player_position == cell_number {
                            cell_label = format!("{:<18}", format!("P{}", player_index + 1));
                            cell_color = match player_index {
                                0 => egui::Color32::BLUE,
                                1 => egui::Color32::GREEN,
                                2 => egui::Color32::YELLOW,
                                3 => egui::Color32::RED,
                                _ => egui::Color32::WHITE,
                            };
                            break;
                        }
                    }

                    // Highlight and annotate snakes and ladders
                    if let Some(&tail) = self.snakes.get(&cell_number) {
                        cell_label = format!("S→{}", tail);
                        cell_width = 20 - cell_label.len();
                        cell_label = format!("{:<cell_width$}", cell_label);
                        cell_color = egui::Color32::DARK_RED;
                    } else if let Some(&top) = self.ladders.get(&cell_number) {
                        cell_label = format!("L→{}", top);
                        cell_width = 20 - cell_label.len();
                        cell_label = format!("{:<cell_width$}", cell_label);
                        cell_color = egui::Color32::DARK_GREEN;
                    }

                    // Render the cell with proper alignment
                    ui.colored_label(cell_color, cell_label);
                }
            });
            ui.add_space(5.0); // Add vertical spacing between rows for better alignment.
        }
    }
}

impl eframe::App for Game {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Snakes and Ladders");

            if !self.game_started {
                ui.label("Select the number of players (2-4):");
                for num_players in 2..=4 {
                    if ui
                        .button(format!("Start with {} players", num_players))
                        .clicked()
                    {
                        self.start_game(num_players);
                    }
                }
            } else if let Some(winner) = self.winner {
                ui.label(format!("Player {} wins!", winner + 1));
            } else {
                ui.label(&self.status_message);

                if ui.button("Roll Dice").clicked() {
                    self.roll_dice();
                }

                ui.separator();
                self.render_board(ui);

                ui.separator();

                for (player_index, position) in self.player_positions.iter().enumerate() {
                    ui.label(format!("Player {}: {}", player_index + 1, position));
                }
            }
        });

        ctx.request_repaint(); // Repaint continuously for smooth updates.
    }
}
