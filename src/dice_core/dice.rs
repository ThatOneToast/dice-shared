use tnet::prelude::*;

#[derive(DPacket, Debug, Clone, Serialize, Deserialize)]
pub struct Dice {
    pub value: u8,
}

impl Dice {
    pub fn new(value: u8) -> Self {
        Self { value }
    }
}

impl Default for Dice {
    fn default() -> Self {
        Self { value: 255 }
    }
}
