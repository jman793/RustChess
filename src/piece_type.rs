use std::fmt;

// Implement your game rules on these enums instead of your other structs
// That way your rendering logic is strict to the driver code and not to the user of your crate
#[derive(Clone, Copy)]
pub enum PieceType {
    KNIGHT,
    QUEEN,
    PAWN,
    BISHOP,
    ROOK,
    KING,
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            PieceType::KNIGHT => "Knight",
            PieceType::QUEEN => "Queen",
            PieceType::PAWN => "Pawn",
            PieceType::BISHOP => "Bishop",
            PieceType::ROOK => "Rook",
            PieceType::KING => "King",
        };
        write!(f, "{}", printable)
    }
}
