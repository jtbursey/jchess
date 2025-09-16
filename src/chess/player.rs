use crate::chess::game::Game;
use crate::chess::r#move::Move;

pub trait Player: PlayerClone {
    fn reset(&mut self);

    fn get_move(&self, _game: &Game) -> Result<Move, String>;

    fn is_human(&self) -> bool {
        return false;
    }

    fn is_bot(&self) -> bool {
        return false;
    }

    fn id_string(&self) -> String {
        return String::from("Player");
    }
}

pub trait PlayerClone {
    fn clone_box(&self) -> Box<dyn Player>;
}

impl<T> PlayerClone for T
where
    T: 'static + Player + Clone
{
    fn clone_box(&self) -> Box<dyn Player> {
        return Box::new(self.clone())
    }
}

impl Clone for Box<dyn Player> {
    fn clone(&self) -> Box<dyn Player> {
        return self.clone_box()
    }
}
