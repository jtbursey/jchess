use crate::chess::game::{Game, StartColor};
use crate::chess::player::Player;

use crate::bots::human::Human;
use crate::bots::bogobot::Bogobot;

//  Title:
//  1. Opponent
//      Title:
//      1. Human
//      2. Bot
//          Title:
//          [List the bots]
//      3. Auto (auto replay a saved game)
//      4. Back
//  2. Color (for playing against a bot, is human black or white)
//      Title:
//      1. Human plays White
//      2. Human plays Black
//      3. Human plays Random
//      4. Back
//  3. Toggle Board Flip
//  4. Back

#[derive(Clone)]
pub struct Setup {
    stack: Vec<SetupMenu>,
    confirm: String,
}

impl Setup {
    pub fn new() -> Self {
        Setup{stack: vec![SetupMenu{ent: Entry::Base, entries: vec![Entry::SelectOpponent, Entry::SelectColor, Entry::DoFlip, Entry::Back]}], confirm: String::new()}
    }

    pub fn current(&self) -> &SetupMenu {
        return self.stack.last().unwrap();
    }

    pub fn select(&mut self, sel: usize, game: &mut Game) -> bool {
        self.confirm = String::new();
        if self.stack.len() <= 0 || sel > self.stack.last().unwrap().entries.len()
        {
            return false;
        }
        match self.stack.last().unwrap().entries[sel] {
            Entry::Base => return false,
            Entry::SelectOpponent => self.select_menu(Entry::SelectOpponent),
            Entry::HumanOpp => self.select_config(Entry::HumanOpp, game),
            Entry::BotOpp => self.select_menu(Entry::BotOpp),
            Entry::Bogobot => self.select_config(Entry::Bogobot, game),
            Entry::Auto => self.select_config(Entry::Auto, game),
            Entry::SelectColor => self.select_menu(Entry::SelectColor),
            Entry::PlayAsWhite => self.select_config(Entry::PlayAsWhite, game),
            Entry::PlayAsBlack => self.select_config(Entry::PlayAsBlack, game),
            Entry::PlayAsRandom => self.select_config(Entry::PlayAsRandom, game),
            Entry::DoFlip => self.select_config(Entry::DoFlip, game),
            Entry::Back => return self.back(),
        };
        return false;
    }

    fn select_menu(&mut self, menu: Entry) {
        match menu {
            Entry::Base => return,
            Entry::SelectOpponent => self.stack.push(SetupMenu{ent: Entry::SelectOpponent, entries: vec![
                Entry::HumanOpp,
                Entry::BotOpp,
                //Entry::Auto,
                Entry::Back
            ]}),
            Entry::BotOpp => self.stack.push(SetupMenu{ent: Entry::BotOpp, entries: vec![
                Entry::Bogobot,
                Entry::Back
            ]}),
            Entry::SelectColor => self.stack.push(SetupMenu{ent: Entry::SelectColor, entries: vec![
                Entry::PlayAsWhite,
                Entry::PlayAsBlack,
                Entry::PlayAsRandom,
                Entry::Back
            ]}),
            _ => return,
        };
    }

    fn select_config(&mut self, menu: Entry, game: &mut Game) {
        match menu {
            Entry::HumanOpp => self.set_opponent(game, Box::new(Human::new())),
            Entry::Bogobot => self.set_opponent(game, Box::new(Bogobot::new())),
            Entry::Auto => return,
            Entry::PlayAsWhite => self.set_start_color(game, StartColor::White),
            Entry::PlayAsBlack => self.set_start_color(game, StartColor::Black),
            Entry::PlayAsRandom => self.set_start_color(game, StartColor::Random),
            Entry::DoFlip => self.toggle_flip(game),
            _ => return,
        }
    }

    fn back(&mut self) -> bool {
        if self.stack.len() <= 1 {
            return true;
        }
        self.stack.pop();
        return false;
    }

    pub fn confirm_string(&self) -> String {
        return String::from(&self.confirm);
    }

    fn toggle_flip(&mut self, game: &mut Game) {
        let res = game.toggle_flip();
        self.confirm = String::from(format!("Toggled Board Flip: {}", if res { "true" } else { "false" }));
    }

    fn set_start_color(&mut self, game: &mut Game, color: StartColor) {
        game.set_start_color(color);
        self.confirm = String::from(format!("Set Player Color: {}", match color{
            StartColor::White => "White",
            StartColor::Black => "Black",
            StartColor::Random => "Random",
        }));
    }

    fn set_opponent(&mut self, game: &mut Game, opp: Box<dyn Player>) {
        self.confirm = String::from(format!("Set Opponent: {}", opp.id_string()));
        game.set_player_two(opp);
    }
}

#[derive(Clone)]
pub struct SetupMenu {
    ent: Entry,                 // Change Opponent
    entries: Vec<Entry>,        // 1. ...
}

impl SetupMenu {
    pub fn this(&self) -> Entry {
        return self.ent;
    }

    pub fn print_entry(&self, idx: usize) -> String {
        if idx >= self.entries.len()
        {
            return String::from("");
        }
        return (idx + 1).to_string() + ". " + self.entries[idx].string().as_str();
    }
}

#[derive(Clone, Copy)]
pub enum Entry {
    Base,
    SelectOpponent,
        HumanOpp,
        BotOpp,
            Bogobot,
        Auto,
    SelectColor,
        PlayAsWhite,
        PlayAsBlack,
        PlayAsRandom,
    DoFlip,
    Back,
}

impl Entry {
    pub fn string(&self) -> String {
        match self {
            Entry::Base => String::from("Setup"),
            Entry::SelectOpponent => String::from("Change Opponent"),
            Entry::HumanOpp => String::from("Play against Human"),
            Entry::BotOpp => String::from("Select a Bot"),
            Entry::Bogobot => String::from("BogoBot"),
            Entry::Auto => String::from("Auto-play"),
            Entry::SelectColor => String::from("Change Color"),
            Entry::PlayAsWhite => String::from("Play as White"),
            Entry::PlayAsBlack => String::from("Play as Black"),
            Entry::PlayAsRandom => String::from("Play as Random"),
            Entry::DoFlip => String::from("Toggle Board Flip"),
            Entry::Back => String::from("Back"),
        }
    }
}
