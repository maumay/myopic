use std::{fmt, ops};

use crate::bitboard::BitBoard;
use crate::square::Square;

impl fmt::Debug for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl ops::Not for Square {
    type Output = BitBoard;

    fn not(self) -> Self::Output {
        !self.loc()
    }
}

impl ops::BitOr<Square> for Square {
    type Output = BitBoard;

    fn bitor(self, other: Square) -> Self::Output {
        self.loc() | other.loc()
    }
}

impl ops::BitOr<BitBoard> for Square {
    type Output = BitBoard;

    fn bitor(self, other: BitBoard) -> Self::Output {
        self.loc() | other
    }
}
