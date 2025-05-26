use std::process::Command;
use std::io;
use std::io::Write;

const TITLE1: &str = "     _  ____ _";
const TITLE2: &str = "    | |/ ___| |__   ___  ___ ___";
const TITLE3: &str = " _  | | |   | '_ \\ / _ \\/ __/ __|";
const TITLE4: &str = "| |_| | |___| | | |  __/\\__ \\__ \\";
const TITLE5: &str = " \\___/ \\____|_| |_|\\___||___/___/";

const LIGHT: &str = "\x1b[48;2;255;193;132m";
const DARK: &str = "\x1b[48;2;110;75;41m";
const HIGHLIGHT1: &str = "\x1b[48;2;200;170;0m";
const HIGHLIGHT2: &str = "\x1b[48;2;200;0;0m";
const RESET: &str = "\x1b[0m";

const NONE: &str = "  ";
const WKING: &str = "\x1b[30m\u{2654} ";
const BKING: &str = "\x1b[30m\u{265a} ";
const WQUEEN: &str = "\x1b[30m\u{2655} ";
const BQUEEN: &str = "\x1b[30m\u{265b} ";
const WROOK: &str = "\x1b[30m\u{2656} ";
const BROOK: &str = "\x1b[30m\u{265c} ";
const WBISHOP: &str = "\x1b[30m\u{2657} ";
const BBISHOP: &str = "\x1b[30m\u{265d} ";
const WKNIGHT: &str = "\x1b[30m\u{2658} ";
const BKNIGHT: &str = "\x1b[30m\u{265e} ";
const WPAWN: &str = "\x1b[30m\u{2659} ";
const BPAWN: &str = "\x1b[30m\u{265f} ";

#[derive(Copy, Clone, Debug, PartialEq)]
struct Rank(Option<u32>);

impl Rank {
    fn new(r: u32) -> Self {
        if r <= 8 && r > 0 { Rank(Some(r)) } else { Rank(None) }
    }

    fn from_index(i: usize) -> Self {
        if i <= 7 { Rank(Some(i as u32 + 1)) } else { Rank(None) }
    }

    fn is_valid(self) -> bool {
        if let Some(r) = self.0
        {
            return r <= 8 && r > 0;
        }
        return false;
    }

    fn index(self) -> Option<usize> {
        if let Some(i) = self.0 { Some(i as usize - 1) } else { None }
    }

    fn to_string(self) -> String {
        if let Some(r) = self.0 { r.to_string() } else { "".to_string() }
    }
}

impl From<Option<char>> for Rank {
    fn from(r: Option<char>) -> Rank {
        if let Some(c) = r 
        {
            let n = c as u32 - '0' as u32;
            if n <= 8 && n > 0
            {
                return Rank(Some(n));
            }
        }
        return Rank(None);
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct File(Option<char>);

impl File {
    fn new(f: char) -> Self {
        if f >= 'a' && f <= 'h' { File(Some(f)) } else { File(None) }
    }

    fn from_index(i: usize) -> Self {
        if i <= 7 { File(Some(('a' as u8 + i as u8) as char)) } else { File(None) }
    }

    fn is_valid(self) -> bool {
        if let Some(f) = self.0
        {
            return f >= 'a' && f <= 'h';
        }
        return false;
    }

    fn index(self) -> Option<usize> {
        if let Some(f) = self.0 { Some(f as usize - 'a' as usize) } else { None }
    }

    fn to_string(self) -> String {
        if let Some(f) = self.0 { f.to_string() } else { "".to_string() }
    }
}

fn tuple_to_square(tuple: (File, Rank)) -> String {
    let (file, rank) = tuple;
    return format!("{}{}", file.to_string(), rank.to_string());
}

fn back_rank_index(c: Color) -> usize {
    return if c == Color::White { 0 } else { 7 };
}

fn promotion_rank_index(c: Color) -> usize {
    return if c == Color::White { 7 } else { 0 };
}

fn rook_castle_file(long_castle: bool) -> usize {
    return if long_castle { 0 } else { 7 };
}

// Going to need a map from PieceKind to the unicode to print

#[derive(Copy, Clone, PartialEq, Eq)]
enum Color {
    White,
    Black,
}

impl Color {
    fn to_string(self) -> String {
        match self
        {
            Color::White => "White".to_string(),
            Color::Black => "Black".to_string(),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum PieceKind {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceKind {
    fn debug_string(self) -> String {
        match self
        {
            PieceKind::None => "None".to_string(),
            PieceKind::Pawn => "Pawn".to_string(),
            PieceKind::Bishop => "Bishop".to_string(),
            PieceKind::Knight => "Knight".to_string(),
            PieceKind::Rook => "Rook".to_string(),
            PieceKind::Queen => "Queen".to_string(),
            PieceKind::King => "King".to_string(),
        }
    }

    fn to_letter(self) -> String {
        match self
        {
            PieceKind::None => "".to_string(),
            PieceKind::Pawn => "".to_string(),
            PieceKind::Bishop => "B".to_string(),
            PieceKind::Knight => "N".to_string(),
            PieceKind::Rook => "R".to_string(),
            PieceKind::Queen => "Q".to_string(),
            PieceKind::King => "K".to_string(),
        }
    }
}

#[derive(Copy, Clone)]
struct Piece {
    kind: PieceKind,
    color: Color,
    has_moved: bool,
    highlight: u8,
}

impl Piece {
    fn make(kind: PieceKind, color: Color, has_moved: bool, highlight: u8) -> Self {
        return Piece{kind: kind, color: color, has_moved: has_moved, highlight: highlight}
    }

    fn get_string(self) -> String {
        let mut s = String::new();
        if self.highlight > 0
        {
            s.push_str(match self.highlight
            {
                1 => HIGHLIGHT1,
                2 => HIGHLIGHT2,
                _ => "",
            });
        }
        s.push_str(match self
        {
            Piece{kind: PieceKind::None, color: _, has_moved: _, highlight: _} => NONE,
            Piece{kind: PieceKind::Pawn, color: Color::White, has_moved: _, highlight: _} => WPAWN,
            Piece{kind: PieceKind::Pawn, color: Color::Black, has_moved: _, highlight: _} => BPAWN,
            Piece{kind: PieceKind::Bishop, color: Color::White, has_moved: _, highlight: _} => WBISHOP,
            Piece{kind: PieceKind::Bishop, color: Color::Black, has_moved: _, highlight: _} => BBISHOP,
            Piece{kind: PieceKind::Knight, color: Color::White, has_moved: _, highlight: _} => WKNIGHT,
            Piece{kind: PieceKind::Knight, color: Color::Black, has_moved: _, highlight: _} => BKNIGHT,
            Piece{kind: PieceKind::Rook, color: Color::White, has_moved: _, highlight: _} => WROOK,
            Piece{kind: PieceKind::Rook, color: Color::Black, has_moved: _, highlight: _} => BROOK,
            Piece{kind: PieceKind::Queen, color: Color::White, has_moved: _, highlight: _} => WQUEEN,
            Piece{kind: PieceKind::Queen, color: Color::Black, has_moved: _, highlight: _} => BQUEEN,
            Piece{kind: PieceKind::King, color: Color::White, has_moved: _, highlight: _} => WKING,
            Piece{kind: PieceKind::King, color: Color::Black, has_moved: _, highlight: _} => BKING,
        });
        return s;
    }

    fn debug_string(self) -> String {
        match self
        {
            Piece{kind: PieceKind::None, color: _, has_moved: _, highlight: _} => "None".to_string(),
            Piece{kind: PieceKind::Pawn, color: _, has_moved: _, highlight: _} => "Pawn".to_string(),
            Piece{kind: PieceKind::Bishop, color: _, has_moved: _, highlight: _} => "Bishop".to_string(),
            Piece{kind: PieceKind::Knight, color: _, has_moved: _, highlight: _} => "Knight".to_string(),
            Piece{kind: PieceKind::Rook, color: _, has_moved: _, highlight: _} => "Rook".to_string(),
            Piece{kind: PieceKind::Queen, color: _, has_moved: _, highlight: _} => "Queen".to_string(),
            Piece{kind: PieceKind::King, color: _, has_moved: _, highlight: _} => "King".to_string(),
        }
    }

    fn to_letter(self) -> String {
        match self
        {
            Piece{kind: PieceKind::None, color: _, has_moved: _, highlight: _} => "".to_string(),
            Piece{kind: PieceKind::Pawn, color: _, has_moved: _, highlight: _} => "".to_string(),
            Piece{kind: PieceKind::Bishop, color: _, has_moved: _, highlight: _} => "B".to_string(),
            Piece{kind: PieceKind::Knight, color: _, has_moved: _, highlight: _} => "N".to_string(),
            Piece{kind: PieceKind::Rook, color: _, has_moved: _, highlight: _} => "R".to_string(),
            Piece{kind: PieceKind::Queen, color: _, has_moved: _, highlight: _} => "Q".to_string(),
            Piece{kind: PieceKind::King, color: _, has_moved: _, highlight: _} => "K".to_string(),
        }
    }

    fn matches(self, other: Piece) -> bool {
        return self.kind == other.kind && self.color == other.color;
    }
}

#[derive(Copy, Clone)]
pub struct Move {
    dest: (File, Rank),
    origin: (File, Rank),
    piece: Piece,
    takes: bool,
    check: bool,
    checkmate: bool,
    castle: bool,
    long_castle: bool,
    pawn_double: bool,
    en_passant: Option<(File, Rank)>,
    promotion: PieceKind,
}

impl Move {
    pub fn new() -> Self {
        Self { dest: (File(None), Rank(None)), origin: (File(None), Rank(None)), piece: Piece{kind: PieceKind::None, color: Color::White, has_moved: false, highlight: 0},
                takes: false, check: false, checkmate: false, castle: false, long_castle: false, pawn_double: false, en_passant: None, promotion:PieceKind::None }
    }

    // For use in checking whether a piece can attack a specific square
    fn basic(p: Piece, o: (File, Rank), d: (File, Rank)) -> Self {
        Self { dest: d, origin: o, piece: p,
                takes: false, check: false, checkmate: false, castle: false, long_castle: false, pawn_double: false, en_passant: None, promotion:PieceKind::None }
    }

    pub fn debug(self) -> String {
        if self.castle
        {
            return "Castles".to_string();
        }
        else if self.long_castle
        {
            return "Long Castles".to_string();
        }
        let o = if tuple_to_square(self.origin).len() > 0 { format!("on {} ", tuple_to_square(self.origin)) } else { "".to_string() };
        let t = if self.takes { "takes on" } else { "to" };
        let p = if self.promotion != PieceKind::None { format!("promotes to {}", self.promotion.debug_string()) } else { "".to_string() };
        let c = if self.check { "with check" } else if self.checkmate { "with checkmate" } else { "" };
        return format!("{} {}{} {} {} {}", self.piece.debug_string(), o, t, tuple_to_square(self.dest), p, c);
    }

    pub fn notation(self) -> String {
        if self.castle
        {
            return "O-O".to_string();
        }
        else if self.long_castle
        {
            return "O-O-O".to_string();
        }
        let t = if self.takes { "x" } else { "" };
        let p = if self.promotion != PieceKind::None { format!("={}", self.promotion.to_letter()) } else { "".to_string() };
        let c = if self.checkmate { "#" } else if self.check { "+" } else { "" };
        return format!("{}{}{}{}{}{}", self.piece.to_letter(), tuple_to_square(self.origin), t, tuple_to_square(self.dest), p, c);
    }
}

#[derive(Copy, Clone)]
pub struct Board([[Piece; 8]; 8]);

#[derive(Copy, Clone)]
enum PrintMode {
    Title,
    Game,
    Checkmate, // also concedes
    Stalemate, // also draw
}

#[derive(Clone)]
pub struct Game {
    board: Board, // board coords are in the order (file, rank)
    to_move: Color,
    turn_count: u32,
    history: Vec<Move>,
    white_cap: Vec<Piece>,
    black_cap: Vec<Piece>,
    error: String,
    print_mode: PrintMode,
}

impl Game {

    // ================
    // Init, default, and reset code
    // ================

    pub fn new() -> Self {
        Self { board: Board{0:[[Piece{kind: PieceKind::None, color: Color::White, has_moved: false, highlight: 0}; 8]; 8]}, to_move: Color::White , turn_count: 1, history: Vec::new(), white_cap: Vec::new(), black_cap: Vec::new(), error: String::new(), print_mode: PrintMode::Title }
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

    pub fn start_game(&mut self) {
        self.default_board();
        self.to_move = Color::White;
        self.turn_count = 1;
        self.print_mode = PrintMode::Game;

    }

    // ================
    // UI Code
    // ================

    pub fn set_error(&mut self, e: String) {
        self.error = format!("Error: {}", e);
        return;
    }

    pub fn clear_notes(&mut self) {
        self.error = String::new();
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

    fn print_rank(&self, r: usize) -> String {
        let mut rank = String::new();
        for f in 0..8
        {
           rank.push_str(&self.board_square(f, r));
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
        let index: usize = self.history.len() - 2*offset + parity;

        let s1 = if index - 1 < self.history.len() { self.history[index - 1].notation() } else { "".to_string() };
        let s2 = if index < self.history.len() { self.history[index].notation() } else { "".to_string() };
        return format!(" {:>3}. {:<10} {:<10}", self.turn_count - offset as u32, s1, s2);
    }

    fn print_title(&self) {
        println!("{: >4}8 {}{: >2}{}", "", self.print_rank(7), "", TITLE1);
        println!("{: >4}7 {}{: >2}{}", "", self.print_rank(6), "", TITLE2);
        println!("{: >4}6 {}{: >2}{}", "", self.print_rank(5), "", TITLE3);
        println!("{: >4}5 {}{: >2}{}", "", self.print_rank(4), "", TITLE4);
        println!("{: >4}4 {}{: >2}{}", "", self.print_rank(3), "", TITLE5);
        println!("{: >4}3 {}{: >2}", "", self.print_rank(2), "",);
        println!("{: >4}2 {}{: >13}Press Enter", "", self.print_rank(1), "");
        println!("{: >4}1 {}{: >2}", "", self.print_rank(0), "");
        println!("{: >4}  a b c d e f g h", "");
        println!("");
        println!("");
        println!("");
    }

    fn print_active_board(&self) {
        println!("{: >4}8 {}{: >3}\u{250c}{:\u{2500}>30}\u{2510}", "", self.print_rank(7), "", "");
        println!("{: >4}7 {}{: >3}\u{2502}{:<30}\u{2502}", "", self.print_rank(6), "", self.print_notation_history(0));
        println!("{: >4}6 {}{: >3}\u{2502}{:<30}\u{2502}", "", self.print_rank(5), "", self.print_notation_history(1));
        println!("{: >4}5 {}{: >3}\u{2502}{:<30}\u{2502}", "", self.print_rank(4), "", self.print_notation_history(2));
        println!("{: >4}4 {}{: >3}\u{2502}{:<30}\u{2502}", "", self.print_rank(3), "", self.print_notation_history(3));
        println!("{: >4}3 {}{: >3}\u{2514}{:\u{2500}>30}\u{2518}", "", self.print_rank(2), "", "");
        println!("{: >4}2 {}{: >3}\x1b[47m{}\x1b[0m", "", self.print_rank(1), "", self.cap_string(Color::Black));
        println!("{: >4}1 {}{: >3}\x1b[47m{}\x1b[0m", "", self.print_rank(0), "", self.cap_string(Color::White));
        println!("{: >4}  a b c d e f g h", "");
    }

    fn print_game(&self) {
        self.print_active_board();

        println!("  \x1b[41m{}\x1b[0m", self.error);

        // Print color based on turn
        println!("{: >6}\u{250c}\u{2500} {} to move {:\u{2500}>12}\u{2510}", "", self.to_move.to_string(), "");
        print!("{: >6}\u{2514} ", "");
        let _ = io::stdout().flush().unwrap();
    }

    fn print_checkmate(&self) {
        self.print_active_board();

        println!("");

        println!("{: >3}\u{250c}{:\u{2500}>30}\u{2510}", "", "");
        println!("{: >3}\u{2502}{:<30}\u{2502}", "", "White Wins");
        println!("{: >3}\u{2514}{:\u{2500}>30}\u{2518}", "", "");

    }

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
            _ => self.print_game(),
        };
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

    // ================
    // Notation Parsing
    // ================

    fn validate_notation(&self, input: &String) -> Option<String> {
        if !input.is_ascii()
        {
            return Some("Non-ascii input".to_string());
        }
        if input.len() < 2
        {
            return Some("Input is too short".to_string());
        }
        if input.len() > 9
        {
            return Some("Input is too long".to_string());
        }
        return None;
    }

    // Assumes a valid 2 length square notation and returns the tuple coords for it.
    fn square_to_tuple(&self, input: &String) -> (File, Rank) {
        let rank = input[1..].parse::<u32>().unwrap();
        let file = input.chars().nth(0).unwrap();
        return (File::new(file), Rank::new(rank));
    }

    fn partial_square_to_tuple(&self, input: &String) -> (File, Rank) {
        if input.len() == 1
        {
            if File(input.chars().nth(0)).is_valid()
            {
                return (File::new(input.chars().nth(0).unwrap()), Rank(None));
            }
            else if Rank::from(input.chars().nth(0)).is_valid()
            {
                return (File(None), Rank::new(input.parse::<u32>().unwrap()))
            }
        }
        else if input.len() == 2 && File(input.chars().nth(0)).is_valid()
                && Rank::from(input.chars().nth(1)).is_valid()
        {
            return self.square_to_tuple(input);
        }

        return (File(None), Rank(None));
    }

    fn find_square_notation_ending(&self, input: &String) -> usize {
        let mut count: usize = 0;

        if input.len() >= 2
        {
            let file = input.chars().nth(input.len() - 2);
            if File(file).is_valid()
            {
                count = count + 1;
            }
        }
        
        if input.len() >= 1
        {
            let rank = input.chars().nth(input.len() - 1); // or rank
            if Rank::from(rank).is_valid() || (count == 0 && File(rank).is_valid())
            {
                count = count + 1;
            }
        }
        
        return count;
    }

    fn validate_square(&self, input: &String) -> bool {
        if input.len() != 2
        {
            return false;
        }

        if self.find_square_notation_ending(input) != 2
        {
            return false;
        }

        let (file, rank) = self.square_to_tuple(input);
        if !rank.is_valid()
        {
            return false;
        }

        if !file.is_valid()
        {
            return false;
        }

        return true;
    }

    // expects a string of length 0 or 1 and returns the pieceKind
    fn notation_find_piece(&self, input: &String) -> Piece {
        if input.is_empty()
        {
            return Piece::make(PieceKind::Pawn, self.to_move, false, 0);
        }

        match input.chars().nth(0).unwrap()
        {
            'P' => Piece::make(PieceKind::Pawn, self.to_move, false, 0),
            'B' => Piece::make(PieceKind::Bishop, self.to_move, false, 0),
            'N' => Piece::make(PieceKind::Knight, self.to_move, false, 0),
            'R' => Piece::make(PieceKind::Rook, self.to_move, false, 0),
            'Q' => Piece::make(PieceKind::Queen, self.to_move, false, 0),
            'K' => Piece::make(PieceKind::King, self.to_move, false, 0),
            _ => Piece::make(PieceKind::None, self.to_move, false, 0),
        }
    }

    fn handle_promotion(&self, input: &mut String, m: &mut Move) {
        if input.len() <= 2
        { // This length is kind of rough. You can't have a valid promotion and destination on only 2 chars
            return;
        }

        let promote: String = input[input.len() - 2..].to_string();
        if promote.chars().nth(0).unwrap() != '='
        {
            return;
        }

        let piece = promote[1..].to_string();
        m.promotion = self.notation_find_piece(&piece).kind;
        let _ = input.split_off(input.len() - 2);
    }

    pub fn parse_notation(&self, mut input: String) -> Result<Move, String> {
        let mut m = Move::new();

        if let Some(s) = self.validate_notation(&input)
        {
            return Err(s);
        }
        
        m.checkmate = input.ends_with("#");
        if m.checkmate
        {
            _ = input.split_off(input.len() - 1);
        }

        m.check = input.ends_with("+");
        if m.check
        {
            _ = input.split_off(input.len() - 1);
        }

        if m.check && m.checkmate
        {
            return Err("Both check and checkmate".to_string());
        }

        // Handle castles after check/mate for those really weird cases.
        if input == "O-O"
        {
            m.castle = true;
            m.piece = Piece{kind: PieceKind::King, color: self.to_move, has_moved: false, highlight: 0};
            return Ok(m);
        }
        else if input == "O-O-O"
        {
            m.long_castle = true;
            m.piece = Piece{kind: PieceKind::King, color: self.to_move, has_moved: false, highlight: 0};
            return Ok(m);
        }

        self.handle_promotion(&mut input, &mut m);

        let dest = input.split_off(input.len() - 2);
        if !self.validate_square(&dest)
        {
            return Err("Invalid square".to_string());
        }
        m.dest = self.square_to_tuple(&dest);

        m.takes = input.ends_with("x");
        if m.takes
        {
            _ = input.split_off(input.len() - 1);
        }

        let square_size = self.find_square_notation_ending(&input);
        let origin = input.split_off(input.len() - square_size);
        m.origin = self.partial_square_to_tuple(&origin);

        m.piece = self.notation_find_piece(&input);

        return Ok(m);
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
        for f in 0..8
        {
            for r in 0..8
            {
                if self.board.0[f][r].kind != PieceKind::None && self.board.0[f][r].color != color
                {
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

    fn is_valid_knight_move(&self, m: &mut Move) -> bool {
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
                            en_passant: m.en_passant, promotion: m.promotion };
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

    pub fn any_valid_moves(&self) -> bool {
        for f in 0..8 {
            for r in 0..8 {
                if self.board.0[f][r].kind != PieceKind::None && self.board.0[f][r].color == self.to_move {
                    for f2 in 0..8 {        // This is a lazy check of all squares.
                        for r2 in 0..8 {    // I'm pretty sure my validation logic is efficient enough
                            let mut m: Move = Move{ dest: (File::from_index(f2), Rank::from_index(r2)), origin: (File::from_index(f), Rank::from_index(r)),
                                            piece: self.board.0[f][r], takes: false, check: false, checkmate: false, castle: false, long_castle: false,
                                            pawn_double: false, en_passant: None,
                                            promotion: if self.board.0[f][r].kind == PieceKind::Pawn { PieceKind::Queen } else { PieceKind::None } };
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
        }
        return false;
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

