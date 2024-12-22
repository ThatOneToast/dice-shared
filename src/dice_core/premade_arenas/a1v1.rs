use crate::prelude::*;
use rand::Rng;
use std::{collections::HashMap, fmt, thread};
use tlogger::prelude::*;
use tnet::prelude::*;

#[derive(DPacket, Debug, Clone, Serialize, Deserialize)]
pub struct OneVOneArena {
    p1: Board3x3,
    p2: Board3x3,
    turn: usize,
    winner: Option<usize>,
    game_over: bool,
}

impl Default for OneVOneArena {
    fn default() -> Self {
        Self {
            p1: Board3x3::default(),
            p2: Board3x3::default(),
            turn: 0,
            winner: None,
            game_over: false,
        }
    }
}

impl DiceArena for OneVOneArena {
    fn new() -> Self {
        Self {
            p1: Board3x3::new(),
            p2: Board3x3::new(),
            turn: 0,
            winner: None,
            game_over: false,
        }
    }

    fn play(&mut self, col: usize, dice: Dice) -> Result<(), DiceError> {
        if self.p1.is_full() || self.p2.is_full() {
            self.game_over = true;

            // Calculate scores
            let p1_score = self.p1.score();
            let p2_score = self.p2.score();

            // Determine winner
            self.winner = if p1_score > p2_score {
                Some(1)
            } else if p2_score > p1_score {
                Some(2)
            } else {
                None // It's a tie
            };

            self.turn = 0;
        }

        let value = dice.value.clone();
        match self.turn {
            1 => {
                self.p1.insert_to(col, dice).map_err(|e| e.into())?;
                self.p2.strike_multi_to(col, value as usize);
                self.turn = 2;
            }
            2 => {
                self.p2.insert_to(col, dice).map_err(|e| e.into())?;
                self.p1.strike_multi_to(col, value as usize);
                self.turn = 1;
            }
            _ => {}
        }

        Ok(())
    }

    fn get_winner(&self) -> Option<usize> {
        return self.winner;
    }

    fn whose_turn(&self) -> usize {
        return self.turn;
    }

    fn is_game_over(&self) -> bool {
        return self.game_over;
    }

    fn score(&self) -> std::collections::HashMap<usize, u32> {
        let p1_score = self.p1.score();
        let p2_score = self.p2.score();

        let mut map = HashMap::new();
        map.insert(1, p1_score);
        map.insert(2, p2_score);
        return map;
    }

    fn simulate(&mut self, delay: std::time::Duration) {
        info!("Arena 1v1 3x3", "Starting game simulation");
        self.turn = 1; // Start with player 1

        while !self.game_over {
            // Generate a random dice roll
            let dice_value = rand::thread_rng().gen_range(1..=6);
            let dice = Dice { value: dice_value };

            // Choose a random column
            let col = rand::thread_rng().gen_range(0..3);

            let current_player = self.turn.to_string();
            info!(
                format!("Simulating Player {}", current_player).as_str(),
                "Rolling dice"
            );

            // Make the play
            let mut current_col = col;
            while let Err(_) = self.play(current_col, dice.clone()) {
                current_col = (current_col + 1) % 3;
                if current_col == col {
                    break; // All columns tried
                }
            }

            debug_box!("Current Arena State", "{}", self);

            println!("{}", self);
            thread::sleep(delay);
        }

        // Game over, display final results
        let scores = self.score();
        info_box!(
            "Game Over!",
            "Final Scores: \n Player 1: {} \n Player 2: {}",
            scores.get(&1).unwrap_or(&0),
            scores.get(&2).unwrap_or(&0)
        );

        match self.winner {
            Some(winner) => info!(format!("Player {}", winner), "Won!"),
            None => info!("Game", "TIE!"),
        }
    }
}

impl fmt::Display for OneVOneArena {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p1_board = format!("{}", self.p1);
        let p2_board = format!("{}", self.p2);

        let p1_lines: Vec<&str> = p1_board.lines().collect();
        let p2_lines: Vec<&str> = p2_board.lines().collect();

        writeln!(
            f,
            "╔════════════════════════╗    ╔════════════════════════╗"
        )?;
        writeln!(f, "║      PLAYER ONE       ║    ║      PLAYER TWO       ║")?;
        writeln!(
            f,
            "╠════════════════════════╣    ╠════════════════════════╣"
        )?;
        for i in 0..p1_lines.len() {
            writeln!(f, "║  {}  ║    ║  {}  ║", p1_lines[i], p2_lines[i])?;
        }
        writeln!(
            f,
            "╚════════════════════════╝    ╚════════════════════════╝"
        )?;
        writeln!(
            f,
            "╔════════════════════════ GAME STATUS ════════════════════════╗"
        )?;
        writeln!(
            f,
            "║           Current Turn: Player {:1}                          ║",
            self.turn
        )?;
        if let Some(winner) = self.winner {
            writeln!(
                f,
                "║           Winner: Player {:1}                                ║",
                winner
            )?;
        }
        writeln!(
            f,
            "╚═══════════════════════════════════════════════════════════════╝"
        )?;
        Ok(())
    }
}
