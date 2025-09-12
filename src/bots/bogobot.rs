use crate::chess::player::*;
use crate::chess::game::Game;
use crate::chess::r#move::Move;

// A silly bot that enumerates all possible moves, then chooses one at random.

#[derive(Clone)]
pub struct Bogobot();

impl Player for Bogobot {
    fn get_move(&self, _game: &Game) -> Result<Move, String> {
        return Ok(Move::new());
    }

    fn is_bot(&self) -> bool {
        return true;
    }
}
