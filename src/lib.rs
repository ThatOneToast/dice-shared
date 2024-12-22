pub mod dice_core;
pub mod general;
pub mod packets;
pub mod prelude;
pub mod sessions;

use tlogger::prelude::*;

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tlogger::{opts::set_debug, text_styling_off};

    use crate::prelude::{DiceArena, OneVOneArena};

    #[test]
    fn one_vers_one_simulation() {
        set_debug(false);
        // text_styling_off();
        let mut arena = OneVOneArena::new();
        arena.simulate(Duration::from_millis(1000));
    }
}

pub mod errors {
    use serde::{Deserialize, Serialize};
    use thiserror::Error;

    #[derive(Error, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum DiceError {
        #[error("Column is full")]
        ColumnFull(),
        #[error("Column {} is invalid", .0)]
        InvalidColumn(usize),
        #[error("Column {} is already occupied", .0)]
        ColumnOccupied(usize),
        #[error("{} Currently already in a session.", .0)]
        JoinMatchMakingWhileInMatch(String),
        #[error("Sent a Matchmaking packet without specifying a GameMode")]
        MatchmakingNoGamemode,
    }
}
