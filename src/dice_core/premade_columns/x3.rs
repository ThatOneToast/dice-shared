use std::fmt;

use crate::dice_core::prelude::*;
use tnet::prelude::*;

#[derive(DPacket, Debug, Clone, Serialize, Deserialize)]
pub struct ColumnX3 {
    values: [Option<Dice>; 3],
}

impl Default for ColumnX3 {
    fn default() -> Self {
        Self {
            values: [None, None, None],
        }
    }
}

impl ColumnX3 {
    pub fn get(&self, index: usize) -> Option<&Dice> {
        if index < self.values.len() {
            self.values[index].as_ref()
        } else {
            None
        }
    }
}

impl DiceColumn for ColumnX3 {
    fn new() -> Self {
        Self {
            values: [None, None, None],
        }
    }

    fn insert(&mut self, dice: Dice) -> Result<(), DiceError> {
        for i in 0..self.values.len() {
            if self.values[i].is_none() {
                self.values[i] = Some(dice);
                return Ok(());
            }
        }
        Err(DiceError::ColumnFull())
    }

    fn strike_multi(&mut self, dice_number: usize) -> Vec<Dice> {
        let mut struck = Vec::new();

        for value in self.values.iter_mut() {
            if let Some(dice) = value {
                if dice.value == dice_number as u8 {
                    if let Some(taken_dice) = value.take() {
                        struck.push(taken_dice);
                    }
                }
            }
        }

        struck
    }

    fn is_full(&self) -> bool {
        self.values.iter().all(|x| x.is_some())
    }

    fn score(&self) -> u32 {
        let mut values: Vec<u32> = self
            .values
            .iter()
            .filter_map(|x| x.as_ref().map(|d| d.value as u32))
            .collect();

        let mut total = 0;
        while !values.is_empty() {
            let current = values[0];
            let count = values.iter().filter(|&&x| x == current).count();

            if count > 1 {
                total += current.pow(count as u32);
            } else {
                total += current;
            }

            values.retain(|&x| x != current);
        }

        total
    }

    fn shift(&mut self) {
        let mut temp_values = Vec::new();

        for value in self.values.iter_mut() {
            if let Some(dice) = value.take() {
                temp_values.push(dice);
            }
        }

        for (i, dice) in temp_values.into_iter().enumerate() {
            self.values[i] = Some(dice);
        }
    }
}

impl fmt::Display for ColumnX3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, value) in self.values.iter().enumerate() {
            match value {
                Some(dice) => write!(f, "{}", dice.value)?,
                None => write!(f, "_")?,
            }
            if i < self.values.len() - 1 {
                write!(f, "|")?;
            }
        }
        write!(f, "]")
    }
}
