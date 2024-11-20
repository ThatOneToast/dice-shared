use crate::errors::DiceError;
use crate::general::GameMode;
use tnet::prelude::*;

#[derive(DPacket, Debug, Clone, Serialize, Deserialize)]
pub struct DicePacket {
    pub action: u8,
    pub game_mode: Option<GameMode>,
    pub is_err: (bool, Option<DiceError>),
    pub game_options: Option<GameOptions>,
}

impl Default for DicePacket {
    fn default() -> Self {
        Self {
            action: 0,
            game_mode: None,
            is_err: (false, None),
            game_options: None,
        }
    }
}

#[derive(DPacket, Debug, Clone, Serialize, Deserialize)]
pub struct GameOptions {
    pub receive_arena_state: (bool, Option<(GameMode, Vec<u8>)>),
    pub play: (bool, Option<(usize, Vec<u8>)>),
}

impl Default for GameOptions {
    fn default() -> Self {
        Self {
            receive_arena_state: (false, None),
            play: (false, None),
        }
    }
}
