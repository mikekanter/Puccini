use super::{Piece, Square};

#[derive(Clone, Copy, PartialEq, Default)]
pub struct FullMove {
    pub piece: Piece,
    pub inner_move: Move,
}

impl FullMove {
    pub const fn new(piece: Piece, mv: Move) -> Self {
        Self {
            piece,
            inner_move: mv,
        }
    }
}


/// A move is essentially represented as a u16.
/// The first 6 bits represent the initial square
/// The last 6 bits represent the target square
/// The last 4 bits represent the MoveType
#[derive(Clone, Copy, PartialEq, Default)]
pub struct Move(pub u16);

/// Basic kinds of moves
/// encoded in 4 bits (promotion, capture, special 1, special 2)
/// See Chess Programming Wiki article on [Encoding Moves](https://www.chessprogramming.org/Encoding_Moves#From-To_Based)
#[derive(Clone, Copy, PartialEq)]
pub enum MoveType {
    Quiet = 0b0000,
    /// double pawn push (allows for en-passant next ply)
    DoublePawnPush = 0b0001,
    // castles
    KingsideCastle = 0b0010,
    QueensideCastle = 0b0011,
    // Captures (non-promotion)
    Capture = 0b0100,
    EnPassant = 0b0101,
    // promotions (non-capture)
    PromotionToKnight = 0b1000,
    PromotionToBishop = 0b1001,
    PromotionToRook = 0b1010,
    PromotionToQueen = 0b1011,
    // promotions (with capture)
    PromotionCaptureToKnight = 0b1100,
    PromotionCaptureToBishop = 0b1101,
    PromotionCaptureToRook = 0b1110,
    PromotionCaptureToQueen = 0b1111,
}

impl Move {
    pub const START_MASK: u16 = 0b0000_0000_0011_1111;
    pub const TARGET_MASK: u16 = 0b0000_1111_1100_0000;
    pub const fn new(start: Square, target: Square, move_type: MoveType) -> Self {
        Self(
            start as u16 | (target as u16) << 6 | (move_type as u16) << 12
        )
    }
    pub const fn start(self) -> Square {
        Square::new((self.0 & Self::START_MASK) as u8)
    }
    pub const fn target(self) -> Square {
        Square::new(((self.0 & Self::TARGET_MASK) >> 6) as u8)
    }
    pub const fn kind(self) -> MoveType {
        unsafe { std::mem::transmute((self.0 >> 12) as u8) }
    }
    pub const fn is_en_passant(self) -> bool {
       matches!(self.kind(), MoveType::EnPassant)
    }
    pub const fn is_castling(self) -> bool {
        matches!(self.kind(), MoveType::KingsideCastle)
            || matches!(self.kind(), MoveType::QueensideCastle)
    }
    /// TODO: See if this actually works.
    pub const fn is_capture(self) -> bool {
        (self.0 >> 14) & 1 != 0
    }
    pub const fn is_promotion(self) -> bool {
        matches!(self.kind(), MoveType::PromotionToKnight)
            || matches!(self.kind(), MoveType::PromotionToBishop)
            || matches!(self.kind(), MoveType::PromotionToRook)
            || matches!(self.kind(), MoveType::PromotionToQueen)
            || matches!(self.kind(), MoveType::PromotionCaptureToKnight)
            || matches!(self.kind(), MoveType::PromotionCaptureToBishop)
            || matches!(self.kind(), MoveType::PromotionCaptureToRook)
            || matches!(self.kind(), MoveType::PromotionCaptureToQueen)
    }
}
