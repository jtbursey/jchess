use crate::chess::piece::*;
use crate::chess::rankfile::*;
use crate::chess::color::Color;

#[derive(Copy, Clone)]
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
