use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Cell {
    T,
    I,
    S,
    Z,
    O,
    L,
    J,
    Empty,
}

// randomization stuff
impl Distribution<Cell> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cell {
        match rng.gen_range(0, 7) {
            0 => Cell::T,
            1 => Cell::I,
            2 => Cell::S,
            3 => Cell::Z,
            4 => Cell::O,
            5 => Cell::L,
            6 => Cell::J,
            _ => Cell::T,
        }
    }
}

impl Cell {
    pub fn to_printable_bytes(&self) -> &[u8] {
        match self {
            Cell::T => b"TTT",
            Cell::I => b"III",
            Cell::S => b"SSS",
            Cell::Z => b"ZZZ",
            Cell::O => b"OOO",
            Cell::L => b"LLL",
            Cell::J => b"JJJ",
            Cell::Empty => b"   ",
        }
    }

    pub fn to_coordinates(&self) -> [u8; 4] {
        match self {
            Self::T => [174, 175, 176, 185],
            Self::I => [175, 185, 195, 205],
            Self::S => [174, 175, 185, 186],
            Self::Z => [175, 176, 184, 185],
            Self::O => [174, 175, 185, 184],
            Self::L => [184, 174, 194, 175],
            Self::J => [185, 175, 195, 174],
            Self::Empty => panic!("The random_new tetromino function must have messed up."),
        }
    }
}
