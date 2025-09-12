use crate::chess::color::Color;
use crate::chess::strings::*;

#[derive(Copy, Clone, PartialEq)]
pub enum PieceKind {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceKind {
    pub fn debug_string(self) -> String {
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

    pub fn to_letter(self) -> String {
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
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
    pub has_moved: bool,
    pub highlight: u8,
}

impl Piece {
    pub fn make(kind: PieceKind, color: Color, has_moved: bool, highlight: u8) -> Self {
        return Piece{kind: kind, color: color, has_moved: has_moved, highlight: highlight}
    }

    pub fn get_string(self) -> String {
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

    pub fn to_letter(self) -> String {
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

    pub fn matches(self, other: Piece) -> bool {
        return self.kind == other.kind && self.color == other.color;
    }
}
