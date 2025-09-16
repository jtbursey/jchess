use crate::chess::color::Color;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rank(pub Option<u32>);

impl Rank {
    pub fn new(r: u32) -> Self {
        if r <= 8 && r > 0 { Rank(Some(r)) } else { Rank(None) }
    }

    pub fn from_index(i: usize) -> Self {
        if i <= 7 { Rank(Some(i as u32 + 1)) } else { Rank(None) }
    }

    pub fn is_valid(self) -> bool {
        if let Some(r) = self.0 {
            return r <= 8 && r > 0;
        }
        return false;
    }

    pub fn index(self) -> Option<usize> {
        if let Some(i) = self.0 { Some(i as usize - 1) } else { None }
    }

    pub fn to_string(self) -> String {
        if let Some(r) = self.0 { r.to_string() } else { "".to_string() }
    }
}

impl From<Option<char>> for Rank {
    fn from(r: Option<char>) -> Rank {
        if let Some(c) = r  {
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
pub struct File(pub Option<char>);

impl File {
    pub fn new(f: char) -> Self {
        if f >= 'a' && f <= 'h' { File(Some(f)) } else { File(None) }
    }

    pub fn from_index(i: usize) -> Self {
        if i <= 7 { File(Some(('a' as u8 + i as u8) as char)) } else { File(None) }
    }

    pub fn is_valid(self) -> bool {
        if let Some(f) = self.0 {
            return f >= 'a' && f <= 'h';
        }
        return false;
    }

    pub fn index(self) -> Option<usize> {
        if let Some(f) = self.0 { Some(f as usize - 'a' as usize) } else { None }
    }

    pub fn to_string(self) -> String {
        if let Some(f) = self.0 { f.to_string() } else { "".to_string() }
    }
}

pub fn tuple_to_square(tuple: (File, Rank)) -> String {
    let (file, rank) = tuple;
    return format!("{}{}", file.to_string(), rank.to_string());
}

pub fn back_rank_index(c: Color) -> usize {
    return if c == Color::White { 0 } else { 7 };
}

pub fn promotion_rank_index(c: Color) -> usize {
    return if c == Color::White { 7 } else { 0 };
}

pub fn rook_castle_file(long_castle: bool) -> usize {
    return if long_castle { 0 } else { 7 };
}
