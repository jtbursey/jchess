use crate::chess::player::*;
use crate::chess::game::Game;
use crate::chess::r#move::Move;

#[derive(Clone)]
pub struct Human();

impl Human {
    pub fn new() -> Self {
        return Human();
    }
}

impl Player for Human {
    fn get_move(&self, _game: &Game) -> Move {
        return Move::new()
    }
}
