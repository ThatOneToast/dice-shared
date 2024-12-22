use crate::dice_core::prelude::*;
use std::fmt;
use tnet::prelude::*;

#[derive(DPacket, Debug, Clone, Serialize, Deserialize)]
pub struct Board3x3 {
    cols: [ColumnX3; 3],
}

impl Default for Board3x3 {
    fn default() -> Self {
        Self {
            cols: [
                ColumnX3::default(),
                ColumnX3::default(),
                ColumnX3::default(),
            ],
        }
    }
}

impl DiceBoard for Board3x3 {
    fn new() -> Self {
        Self {
            cols: [ColumnX3::new(), ColumnX3::new(), ColumnX3::new()],
        }
    }

    fn insert_to(&mut self, col: usize, dice: Dice) -> Result<(), DiceError> {
        if col >= self.cols.len() {
            return Err(DiceError::InvalidColumn(col));
        }
        self.cols[col].insert(dice)
    }

    fn strike_multi_to(&mut self, col: usize, dice_number: usize) -> Vec<Dice> {
        if col >= self.cols.len() {
            return Vec::new();
        }
        self.cols[col].strike_multi(dice_number)
    }

    fn is_full(&self) -> bool {
        self.cols.iter().all(|col| col.is_full())
    }

    fn score(&self) -> u32 {
        self.cols.iter().map(|col| col.score()).sum()
    }
}

impl fmt::Display for Board3x3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "┌───┬───┬───┐")?;
        for row in (0..3).rev() {
            write!(f, "│")?;
            for col in 0..3 {
                match self.cols[col].get(row) {
                    Some(dice) => write!(f, " {} ", dice.value)?,
                    None => write!(f, " · ")?,
                }
                write!(f, "│")?;
            }
            writeln!(f)?;
            if row > 0 {
                writeln!(f, "├───┼───┼───┤")?;
            }
        }
        write!(f, "└───┴───┴───┘")
    }
}
