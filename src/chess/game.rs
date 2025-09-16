use std::process::Command;
use std::io::{Write, stdout};
use rand::Rng;

use crate::chess::color::Color;
use crate::chess::piece::*;
use crate::chess::player::*;
use crate::chess::r#move::*;
use crate::chess::strings::*;
use crate::chess::rankfile::*;
use crate::chess::setup::Setup;

use crate::bots::human::Human;

#[derive(Copy, Clone, PartialEq)]
enum GameMode {
    AgainstHumanLocal,
    AgainstBotLocal,
}

#[derive(Copy, Clone, PartialEq)]
pub enum StartColor {
    White,
    Black,
    Random,
}

#[derive(Copy, Clone)]
enum PrintMode {
    Title,
    Setup,
    Game,
    Checkmate, // also concedes
    Stalemate, // also draw
}

#[derive(Copy, Clone)]
pub struct Board([[Piece; 8]; 8]);

impl Board {
    pub fn new() -> Self {
        Board{0:[[Piece{kind: PieceKind::None, color: Color::White, has_moved: false, highlight: 0}; 8]; 8]}
    }
}

#[derive(Clone)]
struct ActionablePlayer {
    player: Box<dyn Player>,
    color: Color,
}

impl ActionablePlayer {
    fn is_human(&self) -> bool {
        return self.player.is_human()
    }

    fn is_bot(&self) -> bool {
        return self.player.is_bot()
    }

    fn color(&self) -> Color {
        return self.color;
    }

    fn reset(&mut self) {
        self.player.reset();
    }
}

#[derive(Clone)]
pub struct Game {
    board: Board, // board coords are in the order (file, rank)
    to_move: Color,
    player_one: ActionablePlayer,
    player_two: ActionablePlayer,
    turn_count: u32,
    history: Vec<Move>,
    white_cap: Vec<Piece>,
    black_cap: Vec<Piece>,
    error: String,
    orientation: Color,
    do_flip: bool,
    game_mode: GameMode,
    start_color: StartColor,
    print_mode: PrintMode,
}

impl Game {

    // ================
    // Init, default, and reset code
    // ================

    pub fn new() -> Self {
        Self {
            board: Board::new(),
            to_move: Color::White,

            // In Human-Bot matches, human is player one, bot is player two.
            // in Human-Human matches, player one is white, player two is black.
            player_one: ActionablePlayer{ player: Box::new(Human::new()), color: Color::White},
            player_two: ActionablePlayer{ player: Box::new(Human::new()), color: Color::Black},
            turn_count: 1,
            history: Vec::new(),
            white_cap: Vec::new(),
            black_cap: Vec::new(),
            error: String::new(),
            orientation: Color::White,
            do_flip: false,
            game_mode: GameMode::AgainstHumanLocal,
            start_color: StartColor::White,
            print_mode: PrintMode::Title,
        }
    }

    pub fn default_board(&mut self) {
        self.to_move = Color::White;

        let mut c = Color::White;
        self.board.0[0][0] = Piece::make(PieceKind::Rook, c, false, 0);
        self.board.0[1][0] = Piece::make(PieceKind::Knight, c, false, 0);
        self.board.0[2][0] = Piece::make(PieceKind::Bishop, c, false, 0);
        self.board.0[3][0] = Piece::make(PieceKind::Queen, c, false, 0);
        self.board.0[4][0] = Piece::make(PieceKind::King, c, false, 0);
        self.board.0[5][0] = Piece::make(PieceKind::Bishop, c, false, 0);
        self.board.0[6][0] = Piece::make(PieceKind::Knight, c, false, 0);
        self.board.0[7][0] = Piece::make(PieceKind::Rook, c, false, 0);

        for y in 0..8
        {
            self.board.0[y][1] = Piece::make(PieceKind::Pawn, c, false, 0);
        }

        c = Color::Black;
        self.board.0[0][7] = Piece::make(PieceKind::Rook, c, false, 0);
        self.board.0[1][7] = Piece::make(PieceKind::Knight, c, false, 0);
        self.board.0[2][7] = Piece::make(PieceKind::Bishop, c, false, 0);
        self.board.0[3][7] = Piece::make(PieceKind::Queen, c, false, 0);
        self.board.0[4][7] = Piece::make(PieceKind::King, c, false, 0);
        self.board.0[5][7] = Piece::make(PieceKind::Bishop, c, false, 0);
        self.board.0[6][7] = Piece::make(PieceKind::Knight, c, false, 0);
        self.board.0[7][7] = Piece::make(PieceKind::Rook, c, false, 0);

        for y in 0..8
        {
            self.board.0[y][6] = Piece::make(PieceKind::Pawn, c, false, 0);
        }
    }

    pub fn title(&mut self) {
        self.print_mode = PrintMode::Title;
    }

    pub fn start_game(&mut self) {
        self.clear();
        self.default_board();
        self.to_move = Color::White;
        self.turn_count = 1;
        self.print_mode = PrintMode::Game;

        if self.game_mode == GameMode::AgainstBotLocal
        {
            if !self.player_two.is_bot() {
                panic!("Expected a bot but found non-bot!");
            }

            let mut actual_start_color = self.start_color;
            // randomize color if needed
            if actual_start_color == StartColor::Random {
                let mut rng = rand::rng();
                if rng.random_range(0.0..1.0) < 0.5 {
                    actual_start_color = StartColor::White;
                } else {
                    actual_start_color = StartColor::Black;
                }
            }

            // Make the players match the start color
            self.player_one.color = if actual_start_color == StartColor::White { Color::White } else { Color::Black };
            self.player_two.color = if actual_start_color == StartColor::White { Color::Black } else { Color::White };
            // orient based on color
            self.orientation = self.player_one.color();
            // Flipping while playing against a bot is silly.
            self.do_flip = false;
        }
    }

    pub fn start_setup(&mut self) {
        self.print_mode = PrintMode::Setup;
    }

    pub fn clear(&mut self) {
        self.history.clear();
        self.white_cap.clear();
        self.black_cap.clear();
        self.board = Board::new();
        self.to_move = Color::White;
        self.turn_count = 1;
        self.orientation = Color::White;
        self.player_one.reset();
        self.player_one.color = Color::White;
        self.player_two.reset();
        self.player_two.color = Color::Black;
    }

    pub fn toggle_flip(&mut self) -> bool {
        self.do_flip = !self.do_flip;
        return self.do_flip;
    }

    pub fn set_start_color(&mut self, color: StartColor) {
        self.start_color = color;
    }

    // ================
    // Players
    // ================

    pub fn current_player(&self) -> &Box<dyn Player> {
        return self.player(self.to_move);
    }

    pub fn player(&self, color: Color) -> &Box<dyn Player> {
        return if color == self.player_one.color() { &self.player_one.player } else { &self.player_two.player };
    }

    pub fn current_color(&self) -> Color {
        return self.to_move;
    }

    fn update_game_mode(&mut self) {
        if self.player_two.player.is_bot() && self.player_one.player.is_human() {
            self.game_mode = GameMode::AgainstBotLocal;
        }
        else if self.player_two.player.is_human() && self.player_one.player.is_human() {
            self.game_mode = GameMode::AgainstHumanLocal;
        }
    }

    pub fn set_player_one(&mut self, player: Box<dyn Player>) {
        self.player_one.player = player;
        self.update_game_mode();
    }

    pub fn set_player_two(&mut self, player: Box<dyn Player>) {
        self.player_two.player = player;
        self.update_game_mode();
    }

    pub fn set_player_one_color(&mut self, color: Color) {
        self.player_one.color = color;
    }

    pub fn set_player_two_color(&mut self, color: Color) {
        self.player_two.color = color;
    }

    // ================
    // UI Code
    // ================

    pub fn fancy_print(&self) {
        let prelines = 2;

        let mut clear = Command::new("clear");
        let _ = clear.status();

        for _i in 0..prelines
        {   
            println!();
        }
        match self.print_mode
        {
            PrintMode::Title => self.print_title(),
            PrintMode::Game => self.print_game(),
            PrintMode::Checkmate => self.print_checkmate(),
            PrintMode::Stalemate => self.print_stalemate(),
            _ => return,
        };
    }

    pub fn fancy_print_setup(&self, config: &Setup) {
        let prelines = 2;

        let mut clear = Command::new("clear");
        let _ = clear.status();

        for _i in 0..prelines
        {   
            println!();
        }
        match self.print_mode
        {
            PrintMode::Setup => self.print_setup(config),
            _ => return,
        };
    }

    fn print_title(&self) {
        let mut r = if self.orientation == Color::White { 7 } else { 0 };
        println!("{: >5} {}{: >2}{}", r + 1, self.print_rank(r), "", TITLE1);
        r = if self.orientation == Color::White { 6 } else { 1 };
        println!("{: >5} {}{: >2}{}", r + 1, self.print_rank(r), "", TITLE2);
        r = if self.orientation == Color::White { 5 } else { 2 };
        println!("{: >5} {}{: >2}{}", r + 1, self.print_rank(r), "", TITLE3);
        r = if self.orientation == Color::White { 4 } else { 3 };
        println!("{: >5} {}{: >2}{}", r + 1, self.print_rank(r), "", TITLE4);
        r = if self.orientation == Color::White { 3 } else { 4 };
        println!("{: >5} {}{: >2}{}", r + 1, self.print_rank(r), "", TITLE5);
        r = if self.orientation == Color::White { 2 } else { 5 };
        println!("{: >5} {}{: >2}", r + 1, self.print_rank(r), "",);
        r = if self.orientation == Color::White { 1 } else { 6 };
        println!("{: >5} {}{: >7}1. Play ({})", r + 1, self.print_rank(r), "", self.game_mode_string());
        r = if self.orientation == Color::White { 0 } else { 7 };
        println!("{: >5} {}{: >7}2. Setup", r + 1, self.print_rank(r), "");
        println!("{: >5} {}{: >7}3. Exit", "", self.print_rank_label(), "");
        println!("");
        print!("{: >29}> ", "");
        let _ = stdout().flush().unwrap();
    }

    fn print_setup(&self, config: &Setup) {
        let mut r = if self.orientation == Color::White { 7 } else { 0 };
        println!("{: >5} {}{: >7}{}", r + 1, self.print_rank(r), "", config.current().this().string() + ":");
        r = if self.orientation == Color::White { 6 } else { 1 };
        println!("{: >5} {}", r + 1, self.print_rank(r));
        r = if self.orientation == Color::White { 5 } else { 2 };
        println!("{: >5} {}{: >7}{}", r + 1, self.print_rank(r), "", config.current().print_entry(0));
        r = if self.orientation == Color::White { 4 } else { 3 };
        println!("{: >5} {}{: >7}{}", r + 1, self.print_rank(r), "", config.current().print_entry(1));
        r = if self.orientation == Color::White { 3 } else { 4 };
        println!("{: >5} {}{: >7}{}", r + 1, self.print_rank(r), "", config.current().print_entry(2));
        r = if self.orientation == Color::White { 2 } else { 5 };
        println!("{: >5} {}{: >7}{}", r + 1, self.print_rank(r), "", config.current().print_entry(3));
        r = if self.orientation == Color::White { 1 } else { 6 };
        println!("{: >5} {}{: >7}{}", r + 1, self.print_rank(r), "", config.current().print_entry(4));
        r = if self.orientation == Color::White { 0 } else { 7 };
        println!("{: >5} {}{: >7}{}", r + 1, self.print_rank(r), "", config.current().print_entry(5));
        println!("{: >5} {}{: >7}{}", "", self.print_rank_label(), "", "");
        println!("{: >29}{}", "", config.confirm_string());
        print!("{: >29}> ", "");
        let _ = stdout().flush().unwrap();
    }

    fn print_game(&self) {
        self.print_active_board();

        println!("  \x1b[41m{}\x1b[0m", self.error);

        // Print color based on turn
        println!("{: >6}\u{250c}\u{2500} {} to move {:\u{2500}>12}\u{2510}", "", self.to_move.to_string(), "");
        print!("{: >6}\u{2514} ", "");
        let _ = stdout().flush().unwrap();
    }

    fn print_checkmate(&self) {
        self.print_active_board();

        println!("{: >7}\u{250c}{:\u{2500}>12}\u{2510}", "", "");
        println!("{: >7}\u{2502} {} Wins \u{2502}", "", if self.to_move == Color::White { "Black" } else { "White" });
        println!("{: >7}\u{2514}{:\u{2500}>12}\u{2518}", "", "");
    }

    fn print_stalemate(&self) {
        self.print_active_board();

        println!("{: >7}\u{250c}{:\u{2500}>12}\u{2510}", "", "");
        println!("{: >7}\u{2502}    Draw    \u{2502}", "");
        println!("{: >7}\u{2514}{:\u{2500}>12}\u{2518}", "", "");

    }

    fn print_active_board(&self) {
        let mut r = if self.orientation == Color::White { 7 } else { 0 };
        println!("{: >5} {}{: >3}\u{250c}{:\u{2500}>30}\u{2510}", r + 1, self.print_rank(r), "", "");
        r = if self.orientation == Color::White { 6 } else { 1 };
        println!("{: >5} {}{: >3}\u{2502}{:<30}\u{2502}", r + 1, self.print_rank(r), "", self.print_notation_history(0));
        r = if self.orientation == Color::White { 5 } else { 2 };
        println!("{: >5} {}{: >3}\u{2502}{:<30}\u{2502}", r + 1, self.print_rank(r), "", self.print_notation_history(1));
        r = if self.orientation == Color::White { 4 } else { 3 };
        println!("{: >5} {}{: >3}\u{2502}{:<30}\u{2502}", r + 1, self.print_rank(r), "", self.print_notation_history(2));
        r = if self.orientation == Color::White { 3 } else { 4 };
        println!("{: >5} {}{: >3}\u{2502}{:<30}\u{2502}", r + 1, self.print_rank(r), "", self.print_notation_history(3));
        r = if self.orientation == Color::White { 2 } else { 5 };
        println!("{: >5} {}{: >3}\u{2514}{:\u{2500}>30}\u{2518}", r + 1, self.print_rank(r), "", "");
        r = if self.orientation == Color::White { 1 } else { 6 };
        println!("{: >5} {}{: >3}\x1b[47m{}\x1b[0m", r + 1, self.print_rank(r), "", self.cap_string(Color::Black));
        r = if self.orientation == Color::White { 0 } else { 7 };
        println!("{: >5} {}{: >3}\x1b[47m{}\x1b[0m", r + 1, self.print_rank(r), "", self.cap_string(Color::White));
        println!("{: >5} {}", "", self.print_rank_label());
    }

    fn print_rank(&self, r: usize) -> String {
        let mut rank = String::new();
        let mut f: i32 = if self.orientation == Color::White { 0 } else { 7 };
        while f >= 0 && f <= 7
        {
           rank.push_str(&self.board_square(f as usize, r));
           f = if self.orientation == Color::White { f + 1 } else { f - 1 };
        }
        rank.push_str(RESET);
        rank
    }

    fn print_notation_history(&self, offset: usize) -> String {
        if self.history.len() < 2*offset
        {
            return "".to_string();
        }
        let parity = if self.history.len() % 2 == 0 { 1 } else { 0 };
        let index: i32 = self.history.len() as i32 - 2*offset as i32 - parity;
        if index - 1 < 0 {
            return "".to_string();
        }

        let s1 = if index - 1 < self.history.len() as i32 { self.history[index as usize - 1].notation() } else { "".to_string() };
        let s2 = if index < self.history.len() as i32 { self.history[index as usize].notation() } else { "".to_string() };
        return format!(" {:>3}. {:<10} {:<10}", self.turn_count - offset as u32 - parity as u32, s1, s2);
    }

    fn cap_string(&self, c: Color) -> String {
        let mut s = String::new();
        if c == Color::White
        {
            for i in 0..self.white_cap.len()
            {
                s = format!("{}{}", s, self.white_cap[i].get_string());
            }
        }
        else
        {
            for i in 0..self.black_cap.len()
            {
                s = format!("{}{}", s, self.black_cap[i].get_string());
            }
        }

        return s;
    }

    fn print_rank_label(&self) -> String {
        return if self.orientation == Color::White { "a b c d e f g h ".to_string() } else { "h g f e d c b a ".to_string() };
    }

    fn board_square(&self, f: usize, r: usize) -> String {
        let mut space = String::new();
        if (f + r) % 2 == 0 {
            space.push_str(DARK);
        } else {
            space.push_str(LIGHT);
        }

        space.push_str(&self.board.0[f][r].get_string());
        space
    }

    pub fn set_error(&mut self, e: String) {
        self.error = format!("Error: {}", e);
        return;
    }

    pub fn clear_notes(&mut self) {
        self.error = String::new();
    }

    pub fn flip_board(&mut self) {
        self.orientation = if self.orientation == Color::White { Color::Black } else { Color::White };
    }

    pub fn hl_king(&mut self) {
        if let Some((file, rank)) = self.find_king(self.to_move)
        {
            self.board.0[file.index().unwrap()][rank.index().unwrap()].highlight = 2;
        }
    }

    pub fn clear_hl(&mut self) {
        for f in 0..8
        {
            for r in 0..8
            {
                self.board.0[f][r].highlight = 0;
            }
        }
    }

    fn game_mode_string(&self) -> String {
        match self.game_mode {
            GameMode::AgainstHumanLocal => String::from("2 Player Local"),
            GameMode::AgainstBotLocal => String::from("Against Bot"),
        }
    }

    // ================
    // Move Validation
    // ================

    fn find_king(&self, color: Color) -> Option<(File, Rank)> {
        for f in 0..8
        {
            for r in 0..8
            {
                if self.board.0[f][r].kind == PieceKind::King && self.board.0[f][r].color == color
                {
                    return Some((File::from_index(f), Rank::from_index(r)));
                }
            }
        }
        return None;
    }

    // Returns true if any enemy (of the given color) piece is able to attack the given coord
    fn is_attacked(&self, coord: (File, Rank), color: Color) -> bool {
        // for each square, if it is an enemy piece, check if that piece can attack the given square.
        for f in 0..8 {
            for r in 0..8 {
                if self.board.0[f][r].kind != PieceKind::None && self.board.0[f][r].color != color {
                    let mut m = Move::basic(self.board.0[f][r], (File::from_index(f), Rank::from_index(r)), coord);
                    if match self.board.0[f][r].kind {
                        PieceKind::Pawn => self.pawn_attacks(&mut m),
                        PieceKind::Bishop => self.bishop_attacks(&mut m),
                        PieceKind::Knight => self.knight_attacks(&mut m),
                        PieceKind::Rook => self.rook_attacks(&mut m),
                        PieceKind::Queen => self.queen_attacks(&mut m),
                        PieceKind::King => self.king_attacks(&mut m),
                        _ => false,
                    }
                    { return true; }
                }
            }
        }
        return false;
    }

    // returns true if there are no pieces in the range from origin to dest (excluding origin and dest)
    fn is_valid_move_range(&self, m: &Move) -> bool {
        let rdir: i32 = (m.dest.1.index().unwrap() as i32 - m.origin.1.index().unwrap() as i32).signum();
        let fdir: i32 = (m.dest.0.index().unwrap() as i32 - m.origin.0.index().unwrap() as i32).signum();
        let mut f: usize = (m.origin.0.index().unwrap() as i32 + fdir) as usize;
        let mut r: usize = (m.origin.1.index().unwrap() as i32 + rdir) as usize;
        while f != m.dest.0.index().unwrap() || r != m.dest.1.index().unwrap()
        {
            if self.board.0[f][r].kind != PieceKind::None
            {
                return false;
            }
            r = (r as i32 + rdir) as usize;
            f = (f as i32 + fdir) as usize;
        }
        return true;
    }

    // returns true if the move is a valid pawn attack against the dest square
    fn pawn_attacks(&self, m: &mut Move) -> bool {
        let direction : i32 = if m.piece.color == Color::White { 1 } else { -1 };
        let rdiff : i32 = m.dest.1.index().unwrap() as i32 - m.origin.1.index().unwrap() as i32;
        let fdiff: i32 = m.dest.0.index().unwrap() as i32 - m.origin.0.index().unwrap() as i32;
        
        if (fdiff == 1 || fdiff == -1) && rdiff == direction
        { // If capturing, only one to the side, and only one up, and a piece must be there
            if self.board.0[m.dest.0.index().unwrap()][m.dest.1.index().unwrap()].kind != PieceKind::None
            {
                return true;
            }
            else if self.board.0[m.dest.0.index().unwrap()][m.dest.1.index().unwrap()].kind == PieceKind::None
                    && self.history.len() > 0 && self.history[self.history.len() - 1].piece.kind == PieceKind::Pawn
                    && self.history[self.history.len() - 1].dest == (m.dest.0, Rank::from_index((m.dest.1.index().unwrap() as i32 - direction) as usize))
                    && self.history[self.history.len() - 1].pawn_double
            {
                m.en_passant = Some((m.dest.0, Rank::from_index((m.dest.1.index().unwrap() as i32 - direction) as usize)));
                return true;
            }
        }
        return false;
    }

    fn pawn_moves(&self, m: &mut Move) -> bool {
        let direction: i32 = if m.piece.color == Color::White { 1 } else { -1 };
        let rdiff: i32 = m.dest.1.index().unwrap() as i32 - m.origin.1.index().unwrap() as i32;
        let fdiff: i32 = m.dest.0.index().unwrap() as i32 - m.origin.0.index().unwrap() as i32;
        if fdiff == 0 && self.board.0[m.dest.0.index().unwrap()][m.dest.1.index().unwrap()].kind == PieceKind::None
        { // If moving forward, cannot capture. Also cannot move backwards
            if rdiff == direction
            {
                return true;
            }
            else if rdiff == 2*direction && self.board.0[m.origin.0.index().unwrap()][m.origin.1.index().unwrap()].has_moved == false
            {
                m.pawn_double = true;
                let file = m.origin.0.index().unwrap();
                let rank = (m.origin.1.index().unwrap() as i32 + direction) as usize;
                return self.board.0[file][rank].kind == PieceKind::None;
            }
        }
        return false;
    }

    fn is_valid_promotion(&self, m: &Move) -> bool {
        // if there is no promotion and the pawn did not get to the end rank, default to true.
        if m.promotion == PieceKind::None && (m.piece.kind != PieceKind::Pawn || m.dest.1.index().unwrap() != promotion_rank_index(m.piece.color))
        {
            return true;
        }
        // if a promotion is specified, check that a pawn is getting to the end rank
        if m.piece.kind == PieceKind::Pawn && m.dest.1.index().unwrap() == promotion_rank_index(m.piece.color)
        {
            return match m.promotion
            {
                PieceKind::Knight => true,
                PieceKind::Bishop => true,
                PieceKind::Rook => true,
                PieceKind::Queen => true,
                _ => false,
            };
        }
        return false;
    }

    fn is_valid_pawn_move(&self, m: &mut Move) -> bool {
        return (self.pawn_attacks(m) || self.pawn_moves(m)) && self.is_valid_promotion(m);
    }

    fn bishop_attacks(&self, m: &Move) -> bool {
        let rdiff: i32 = (m.dest.1.index().unwrap() as i32 - m.origin.1.index().unwrap() as i32).abs();
        let fdiff: i32 = (m.dest.0.index().unwrap() as i32 - m.origin.0.index().unwrap() as i32).abs();
        if (rdiff == 0 && fdiff == 0) || rdiff != fdiff
        {
            return false;
        }
        return self.is_valid_move_range(m);
    }

    fn is_valid_bishop_move(&self, m: &Move) -> bool {
        return self.bishop_attacks(m);
    }

    fn knight_attacks(&self, m: &Move) -> bool {
        let rdiff: i32 = (m.dest.1.index().unwrap() as i32 - m.origin.1.index().unwrap() as i32).abs();
        let fdiff: i32 = (m.dest.0.index().unwrap() as i32 - m.origin.0.index().unwrap() as i32).abs();
        return (rdiff == 2 && fdiff == 1) || (rdiff == 1 && fdiff == 2);
    }

    fn is_valid_knight_move(&self, m: &Move) -> bool {
        return self.knight_attacks(m);
    }

    fn rook_attacks(&self, m: &Move) -> bool {
        let rdiff: i32 = m.dest.1.index().unwrap() as i32 - m.origin.1.index().unwrap() as i32;
        let fdiff: i32 = m.dest.0.index().unwrap() as i32 - m.origin.0.index().unwrap() as i32;
        // first is technically redundant because of the check in is_valid_move()
        if rdiff + fdiff == 0 || (rdiff != 0 && fdiff != 0)
        {
            return false;
        }
        return self.is_valid_move_range(m);
    }

    fn is_valid_rook_move(&self, m: &Move) -> bool {
        return self.rook_attacks(m);
    }

    fn queen_attacks(&self, m: &Move) -> bool {
        return self.rook_attacks(m) || self.bishop_attacks(m);
    }

    fn is_valid_queen_move(&self, m: &Move) -> bool {
        return self.queen_attacks(m);
    }

    fn king_attacks(&self, m: &Move) -> bool {
        let rdiff: i32 = (m.dest.1.index().unwrap() as i32 - m.origin.1.index().unwrap() as i32).abs();
        let fdiff: i32 = (m.dest.0.index().unwrap() as i32 - m.origin.0.index().unwrap() as i32).abs();
        return rdiff + fdiff > 0 && rdiff >= 0 && rdiff <= 1 && fdiff >= 0 && fdiff <= 1;
    }

    fn is_valid_king_move(&self, m: &Move) -> bool {
        return self.king_attacks(m);
    }

    fn is_valid_castle(&self, m: &Move) -> Option<String> {
        // Check that the king is not in check, that the squares it will pass through are not attacked,
        // and that the king and rook have not moved
        // First, check that all the pieces are in place and have not moved, and the king is not in check
        if self.board.0[4][back_rank_index(self.to_move)].kind != PieceKind::King || self.board.0[4][back_rank_index(self.to_move)].has_moved
            || self.board.0[rook_castle_file(m.long_castle)][back_rank_index(self.to_move)].kind != PieceKind::Rook
            || self.board.0[rook_castle_file(m.long_castle)][back_rank_index(self.to_move)].has_moved
            || self.is_attacked((File::from_index(4), Rank::from_index(back_rank_index(self.to_move))), self.to_move)
        {
            return Some("King/Rook are not valid".to_string());
        }

        // Now check the in-between squares for peices or checks
        if m.long_castle
        {
            if self.board.0[3][back_rank_index(self.to_move)].kind != PieceKind::None
                || self.is_attacked((File::from_index(3), Rank::from_index(back_rank_index(self.to_move))), self.to_move)
                || self.board.0[2][back_rank_index(self.to_move)].kind != PieceKind::None
                || self.is_attacked((File::from_index(2), Rank::from_index(back_rank_index(self.to_move))), self.to_move)
            {
                return Some("Castle path is not clear".to_string());
            }
        }
        else
        {
            if self.board.0[5][back_rank_index(self.to_move)].kind != PieceKind::None
                || self.is_attacked((File::from_index(5), Rank::from_index(back_rank_index(self.to_move))), self.to_move)
                || self.board.0[6][back_rank_index(self.to_move)].kind != PieceKind::None
                || self.is_attacked((File::from_index(6), Rank::from_index(back_rank_index(self.to_move))), self.to_move)
            {
                return Some("Castle path is not clear".to_string());
            }
        }

        return None;
    }

    fn is_valid_move(&self, m: &mut Move) -> Option<String> {
        // For castles we can't check dest. we only have long/castles and a piece (the king)
        if m.castle || m.long_castle
        {
            return self.is_valid_castle(m);
        }

        // is there a piece of the same color at the destination?
        if self.board.0[m.dest.0.index().unwrap()][m.dest.1.index().unwrap()].kind != PieceKind::None
            && self.board.0[m.dest.0.index().unwrap()][m.dest.1.index().unwrap()].color == m.piece.color
        {
            return Some("There is a piece at the destination".to_string());
        }
        else if m.dest == m.origin
        {
            return Some("Origin and Destination are the same".to_string());
        }

        if self.board.0[m.dest.0.index().unwrap()][m.dest.1.index().unwrap()].kind != PieceKind::None
            && self.board.0[m.dest.0.index().unwrap()][m.dest.1.index().unwrap()].color != m.piece.color
        {
            m.takes = true;
        }
        else if m.takes
        {
            return Some("There is no piece to take".to_string());
        }

        return if match m.piece.kind {
            PieceKind::Pawn => self.is_valid_pawn_move(m),
            PieceKind::Bishop => self.is_valid_bishop_move(m),
            PieceKind::Knight => self.is_valid_knight_move(m),
            PieceKind::Rook => self.is_valid_rook_move(m),
            PieceKind::Queen => self.is_valid_queen_move(m),
            PieceKind::King => self.is_valid_king_move(m),
            _ => false,
        } { None } else { Some("Selected piece cannot make that move".to_string()) };
    }

    // disambigutate the origin piece in the move. Modify the Move object to reflect it.
    pub fn disambiguate(&self, m: &mut Move) -> Option<String> {
        if m.castle || m.long_castle
        {
            return if let Some(s) = self.is_valid_castle(m) { Some(s) } else { None };
        }

        let mut ambi = Vec::new();
        let (file, rank) = m.origin;
        if file.is_valid() && rank.is_valid()
        {
            // check square -> do nothing
            ambi.push((file, rank));
        }
        else if file.is_valid()
        {
            for r in 0..8
            {
                if self.board.0[file.index().unwrap()][r].matches(m.piece)
                {
                    ambi.push((file, Rank::from_index(r)));
                }
            }
        }
        else if rank.is_valid()
        {
            for f in 0..8
            {
                if self.board.0[f][rank.index().unwrap()].matches(m.piece)
                {
                    ambi.push((File::from_index(f), rank));
                }
            }
        }
        else
        {
            // check board
            for f in 0..8
            {
                for r in 0..8
                {
                    if self.board.0[f][r].matches(m.piece)
                    {
                        ambi.push((File::from_index(f), Rank::from_index(r)));
                    }
                }
            }
        }
        if ambi.len() == 0
        {
            return Some("No pieces match".to_string());
        }

        // For each remaining piece, check if they have a valid move to the destination.
        // i.e. does the piece move like that to get there
        let mut ambi2 = Vec::new();
        for i in 0..ambi.len()
        {
            let mut cur = Move{ dest: m.dest, origin: ambi[i], piece: self.board.0[ambi[i].0.index().unwrap()][ambi[i].1.index().unwrap()],
                            takes: m.takes, check: m.check, checkmate: m.checkmate, castle: m.castle, long_castle: m.long_castle, pawn_double: m.pawn_double,
                            en_passant: m.en_passant, promotion: m.promotion, meta: MetaMove::None };
            if self.is_valid_move(&mut cur) == None
            {
                ambi2.push(cur);
            }
        }
        
        if ambi2.len() == 1
        {
            *m = ambi2[0];
            return None;
        }

        return Some("No pieces can make that move".to_string());
    }

    fn is_check_color(&self, color: Color) -> bool {
        if let Some((file, rank)) = self.find_king(color)
        {
            return self.is_attacked((file, rank), color);
        }
        
        // This should not happen... Famous last words
        return false;
    }

    pub fn is_check(&self) -> bool {
        return self.is_check_color(self.to_move);
    }

    pub fn all_piece_moves(&self, file: File, rank: Rank) -> Vec<Move> {
        let piece: Piece = self.board.0[file.index().unwrap()][rank.index().unwrap()];
        return match piece.kind {
            PieceKind::Pawn => gen_pawn_moves(piece, file, rank),
            PieceKind::Bishop => gen_bishop_moves(piece, file, rank),
            PieceKind::Knight => gen_knight_moves(piece, file, rank),
            PieceKind::Rook => gen_rook_moves(piece, file, rank),
            PieceKind::Queen => gen_queen_moves(piece, file, rank),
            PieceKind::King => gen_king_moves(piece, file, rank),
            PieceKind::None => Vec::<Move>::new(),
        };
    }

    pub fn any_valid_moves(&self) -> bool {
        for f in 0..8 {
            for r in 0..8 {
                if self.board.0[f][r].kind != PieceKind::None && self.board.0[f][r].color == self.to_move {
                    // enumerate moves by piece
                    let piece_moves = self.all_piece_moves(File::from_index(f), Rank::from_index(r));
                    // check if the moves are valid
                    for mut m in piece_moves
                    {
                        if self.is_valid_move(&mut m) == None {
                            let mut temp = self.clone();
                            temp.do_move(m);
                            if !temp.is_check() {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        return false;
    }

    pub fn list_valid_moves(&self) -> Vec<Move> {
        let mut moves : Vec<Move> = vec![];
        for f in 0..8 {
            for r in 0..8 {
                if self.board.0[f][r].kind != PieceKind::None && self.board.0[f][r].color == self.to_move {
                    // enumerate moves by piece
                    let piece_moves = self.all_piece_moves(File::from_index(f), Rank::from_index(r));
                    // check if the moves are valid
                    for mut m in piece_moves
                    {
                        if self.is_valid_move(&mut m) == None {
                            let mut temp = self.clone();
                            temp.do_move(m);
                            if !temp.is_check() {
                                moves.push(m);
                            }
                        }
                    }
                }
            }
        }

        // Check castles
        for mut m in gen_castles() {
            if self.is_valid_move(&mut m) == None {
                let mut temp = self.clone();
                temp.do_move(m);
                if !temp.is_check() {
                    moves.push(m);
                }
            }
        }
        return moves;
    }

    // ================
    // Move Action
    // ================

    // Takes a valid move, and performs it
    pub fn do_move(&mut self, mut m: Move) -> Self {
        let previous_state = self.clone();
        if m.castle || m.long_castle
        {
            let king_file_dest: usize = if m.castle { 6 } else { 2 };
            let rook_file_dest: usize = if m.castle { 5 } else { 3 };
            let rook_file_origin: usize = if m.castle { 7 } else { 0 };

            self.board.0[king_file_dest][back_rank_index(self.to_move)] = self.board.0[4][back_rank_index(self.to_move)];
            self.board.0[king_file_dest][back_rank_index(self.to_move)].has_moved = true;
            self.board.0[4][back_rank_index(self.to_move)] = Piece{kind: PieceKind::None, color: Color::White, has_moved: false, highlight: 1};

            self.board.0[rook_file_dest][back_rank_index(self.to_move)] = self.board.0[rook_file_origin][back_rank_index(self.to_move)];
            self.board.0[rook_file_dest][back_rank_index(self.to_move)].has_moved = true;
            self.board.0[rook_file_origin][back_rank_index(self.to_move)] = Piece{kind: PieceKind::None, color: Color::White, has_moved: false, highlight: 1};
        }
        else
        {
            if m.takes
            {
                if self.to_move == Color::White
                {
                    self.white_cap.push(self.board.0[m.dest.0.index().unwrap()][m.dest.1.index().unwrap()]);
                }
                else
                {
                    self.black_cap.push(self.board.0[m.dest.0.index().unwrap()][m.dest.1.index().unwrap()]);
                }
            }
            self.board.0[m.dest.0.index().unwrap()][m.dest.1.index().unwrap()] = self.board.0[m.origin.0.index().unwrap()][m.origin.1.index().unwrap()];
            self.board.0[m.dest.0.index().unwrap()][m.dest.1.index().unwrap()].has_moved = true;
            self.board.0[m.dest.0.index().unwrap()][m.dest.1.index().unwrap()].highlight = 1;
            self.board.0[m.origin.0.index().unwrap()][m.origin.1.index().unwrap()] = Piece{kind: PieceKind::None, color: Color::White, has_moved: false, highlight: 1};
            if let Some((file, rank)) = m.en_passant
            {
                self.board.0[file.index().unwrap()][rank.index().unwrap()] = Piece{ kind: PieceKind::None, color: Color::White, has_moved: false, highlight: 0 };
            }
            if m.promotion != PieceKind::None
            {
                self.board.0[m.dest.0.index().unwrap()][m.dest.1.index().unwrap()].kind = m.promotion;
            }
        }

        if self.is_check_color(if self.to_move == Color::White { Color::Black } else { Color::White })
        {
            m.check = true;
        }
        self.history.push(m);
        return previous_state;
    }

    pub fn next_turn(&mut self) {
        self.to_move = if self.to_move == Color::White { Color::Black } else { Color::White };
        self.turn_count += if self.to_move == Color::White { 1 } else { 0 };
        if self.do_flip {
            self.orientation = self.to_move;
        }
    }

    pub fn set_checkmate(&mut self) {
        self.print_mode = PrintMode::Checkmate;
        if self.history.len() > 0
        {
            let index = self.history.len() - 1;
            self.history[index].checkmate = true;
            self.history[index].check = false;
        }
        self.hl_king();
    }

    pub fn set_stalemate(&mut self) {
        self.print_mode = PrintMode::Stalemate;
    }

    pub fn set_concede(&mut self) {
        self.print_mode = PrintMode::Checkmate;
    }
}
