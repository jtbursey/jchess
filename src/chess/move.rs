use crate::chess::piece::*;
use crate::chess::rankfile::*;
use crate::chess::color::Color;

#[derive(Copy, Clone, PartialEq)]
pub enum MetaMove {
    None,
    Quit,
    Concede,
    Flip,
}

#[derive(Copy, Clone)]
pub struct Move {
    pub dest: (File, Rank),
    pub origin: (File, Rank),
    pub piece: Piece,
    pub takes: bool,
    pub check: bool,
    pub checkmate: bool,
    pub castle: bool,
    pub long_castle: bool,
    pub pawn_double: bool,
    pub en_passant: Option<(File, Rank)>,
    pub promotion: PieceKind,
    pub meta: MetaMove,
}

impl Move {
    pub fn new() -> Self {
        Self { dest: (File(None), Rank(None)), origin: (File(None), Rank(None)), piece: Piece{kind: PieceKind::None, color: Color::White, has_moved: false, highlight: 0},
                takes: false, check: false, checkmate: false, castle: false, long_castle: false, pawn_double: false, en_passant: None, promotion:PieceKind::None, meta: MetaMove::None }
    }

    // For use in checking whether a piece can attack a specific square
    pub fn basic(p: Piece, o: (File, Rank), d: (File, Rank)) -> Self {
        Self { dest: d, origin: o, piece: p,
                takes: false, check: false, checkmate: false, castle: false, long_castle: false, pawn_double: false, en_passant: None, promotion:PieceKind::None, meta: MetaMove::None }
    }

    pub fn new_meta(m: MetaMove) -> Self {
        Self { dest: (File(None), Rank(None)), origin: (File(None), Rank(None)), piece: Piece{kind: PieceKind::None, color: Color::White, has_moved: false, highlight: 0},
                takes: false, check: false, checkmate: false, castle: false, long_castle: false, pawn_double: false, en_passant: None, promotion:PieceKind::None, meta: m }
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
        return format!("{} {}{} {} {} {}", self.piece.kind.debug_string(), o, t, tuple_to_square(self.dest), p, c);
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

// ================
// Notation Parsing
// ================

fn validate_notation(input: &String) -> Option<String> {
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
fn square_to_tuple(input: &String) -> (File, Rank) {
    let rank = input[1..].parse::<u32>().unwrap();
    let file = input.chars().nth(0).unwrap();
    return (File::new(file), Rank::new(rank));
}

fn partial_square_to_tuple(input: &String) -> (File, Rank) {
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
        return square_to_tuple(input);
    }

    return (File(None), Rank(None));
}

fn find_square_notation_ending(input: &String) -> usize {
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

fn validate_square(input: &String) -> bool {
    if input.len() != 2
    {
        return false;
    }

    if find_square_notation_ending(input) != 2
    {
        return false;
    }

    let (file, rank) = square_to_tuple(input);
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
fn notation_find_piece(input: &String, to_move: Color) -> Piece {
    if input.is_empty()
    {
        return Piece::make(PieceKind::Pawn, to_move, false, 0);
    }

    match input.chars().nth(0).unwrap()
    {
        'P' => Piece::make(PieceKind::Pawn, to_move, false, 0),
        'B' => Piece::make(PieceKind::Bishop, to_move, false, 0),
        'N' => Piece::make(PieceKind::Knight, to_move, false, 0),
        'R' => Piece::make(PieceKind::Rook, to_move, false, 0),
        'Q' => Piece::make(PieceKind::Queen, to_move, false, 0),
        'K' => Piece::make(PieceKind::King, to_move, false, 0),
        _ => Piece::make(PieceKind::None, to_move, false, 0),
    }
}

fn handle_promotion(input: &mut String, m: &mut Move, to_move: Color) {
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
    m.promotion = notation_find_piece(&piece, to_move).kind;
    let _ = input.split_off(input.len() - 2);
}

pub fn parse_notation(mut input: String, to_move: Color) -> Result<Move, String> {
    let mut m = Move::new();

    // Handle special commands
    if input == "quit" || input == "exit" {
        return Ok(Move::new_meta(MetaMove::Quit));
    } else if input == "concede" {
        return Ok(Move::new_meta(MetaMove::Concede));
    } else if input == "flip" {
        return Ok(Move::new_meta(MetaMove::Flip));
    }

    if let Some(s) = validate_notation(&input)
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
        m.piece = Piece{kind: PieceKind::King, color: to_move, has_moved: false, highlight: 0};
        return Ok(m);
    }
    else if input == "O-O-O"
    {
        m.long_castle = true;
        m.piece = Piece{kind: PieceKind::King, color: to_move, has_moved: false, highlight: 0};
        return Ok(m);
    }

    handle_promotion(&mut input, &mut m, to_move);

    let dest = input.split_off(input.len() - 2);
    if !validate_square(&dest)
    {
        return Err("Invalid square".to_string());
    }
    m.dest = square_to_tuple(&dest);

    m.takes = input.ends_with("x");
    if m.takes
    {
        _ = input.split_off(input.len() - 1);
    }

    let square_size = find_square_notation_ending(&input);
    let origin = input.split_off(input.len() - square_size);
    m.origin = partial_square_to_tuple(&origin);

    m.piece = notation_find_piece(&input, to_move);

    return Ok(m);
}
