pub mod dice_core;
pub mod general;
pub mod packets;
pub mod prelude;
pub mod sessions;

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
