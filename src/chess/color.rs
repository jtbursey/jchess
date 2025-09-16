#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn to_string(self) -> String {
        match self {
            Color::White => "White".to_string(),
            Color::Black => "Black".to_string(),
        }
    }
}
