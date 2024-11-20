use tnet::prelude::*;
use uuid::Uuid;

pub struct MatchData;

#[derive(DSession, Debug, Clone, Serialize, Deserialize)]
pub struct DiceSession {
    #[session_id]
    pub id: String,
    pub username: String,
    pub in_match: bool,
    pub in_match_making: bool,
    pub game_id: Option<String>,
}

impl Default for DiceSession {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            username: "DEFAULT".to_string(),
            in_match: false,
            in_match_making: false,
            game_id: None,
        }
    }
}
