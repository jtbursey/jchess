use crate::chess::game::Game;
use crate::chess::player::*;
use crate::chess::r#move::*;
use crate::input::*;

// Here for human input

#[derive(Clone)]
pub struct Human();

impl Human {
    pub fn new() -> Self {
        return Human();
    }
}

impl Player for Human {
    fn reset(&mut self) {}

    fn get_move(&self, game: &Game) -> Result<Move, String> {
        let input = read_line();

        return match parse_notation(input, game.current_color()) {
            Ok(m) => Ok(m),
            Err(s) => Err(s),
        };
    }

    fn is_human(&self) -> bool {
        return true;
    }

    fn id_string(&self) -> String {
        return String::from("Human");
    }
}
