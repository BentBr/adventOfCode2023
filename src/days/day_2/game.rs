use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Game {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Game {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Game { red, green, blue }
    }

    // Associated function to provide default values
    pub fn default() -> Self {
        Game {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

lazy_static! {
    pub static ref GAME_CONFIGURATION: Game = Game::new(12, 13, 14);
}
