use crate::chess::player::*;
use crate::chess::game::Game;
use crate::chess::r#move::Move;

use rand::Rng;

// A silly bot that enumerates all possible moves, then chooses one at random.

#[derive(Clone)]
pub struct Bogobot();

impl Bogobot {
    pub fn new() -> Self {
        return Bogobot();
    }
}

impl Player for Bogobot {
    fn reset(&mut self)  {}

    fn get_move(&self, game: &Game) -> Result<Move, String> {
        let all_moves = game.list_valid_moves();
        if all_moves.len() == 0 {
            return Err(String::from("No valid moves for BogoBot!"));
        }
        let mut rng = rand::rng();
        let choice = rng.random_range(0..all_moves.len());
        return Ok(all_moves[choice]);
    }

    fn is_bot(&self) -> bool {
        return true;
    }

    fn id_string(&self) -> String {
        return String::from("BogoBot");
    }
}
