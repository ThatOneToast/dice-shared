use crate::dice_core::prelude::Dice;
use crate::prelude::*;
use std::{collections::HashMap, time::Duration};

pub trait DiceColumn {
    fn new() -> Self;
    fn insert(&mut self, dice: Dice) -> Result<(), DiceError>;
    fn strike_multi(&mut self, dice_number: usize) -> Vec<Dice>;
    fn is_full(&self) -> bool;
    fn score(&self) -> u32;
    fn shift(&mut self);
}

pub trait DiceBoard {
    fn new() -> Self;
    fn insert_to(&mut self, col: usize, dice: Dice) -> Result<(), DiceError>;
    fn strike_multi_to(&mut self, col: usize, dice_number: usize) -> Vec<Dice>;
    fn is_full(&self) -> bool;
    fn score(&self) -> u32;
}

pub trait DiceArena {
    fn new() -> Self;
    fn play(&mut self, col: usize, dice: Dice) -> Result<(), DiceError>;
    fn get_winner(&self) -> Option<usize>;
    fn whose_turn(&self) -> usize;
    fn is_game_over(&self) -> bool;
    fn score(&self) -> HashMap<usize, u32>;
    fn simulate(&mut self, delay: Duration);
}
