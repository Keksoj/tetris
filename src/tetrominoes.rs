// This part of the library provides one thing, and one thing only :
// Tetromino instances.
// Create them, push them, turn them, get new coordinates,
// but DON'T make them interact with the frame !

extern crate rand;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug, Clone)]
pub struct Tetromino {
    pub blocks: [[u8; 2]; 4], // 4 sets of coordinates [height, abscissa]
    pub name: Name,
    pub orientation: Orientation,
    pub footprint: u8,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Orientation {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone)]
pub enum Name {
    // Here are the footprints:
    T, // 1
    I, // 2
    S, // 3
    Z, // 4
    O, // 5
    L, // 6
    J, // 7
}

// Some whirly-dirly randomization stuff around chosing the next tetromino
impl Distribution<Name> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Name {
        match rng.gen_range(0, 7) {
            0 => Name::T,
            1 => Name::I,
            2 => Name::S,
            3 => Name::Z,
            4 => Name::O,
            5 => Name::L,
            6 => Name::J,
            _ => Name::T,
        }
    }
}

impl Tetromino {
    pub fn randow_new_tetromino() -> Self {
        let name: Name = rand::random(); // where does this random() come from ?
        Tetromino::create(name)
    }

    pub fn pull_down(&mut self) {
        for n in 0..3 {
            self.blocks[n][0] -= 1;
        }
    }

    pub fn push_left(&mut self) {
        for n in 0..3 {
            self.blocks[n][1] -= 1;
        }
    }
    pub fn push_right(&mut self) {
        for n in 0..3 {
            self.blocks[n][1] += 1;
        }
    }

    pub fn create(name: Name) -> Self {
        match name {
            Name::I => Tetromino::create_i(),
            Name::T => Tetromino::create_t(),
            Name::S => Tetromino::create_s(),
            Name::Z => Tetromino::create_z(),
            Name::O => Tetromino::create_o(),
            Name::L => Tetromino::create_t(),
            Name::J => Tetromino::create_t(),
            // "O" => create_o(),
        }
    }

    // T, footprint 1
    pub fn create_t() -> Self {
        Self {
            // first one doesn't move
            blocks: [[18, 5], [18, 4], [19, 5], [18, 6]],
            name: Name::T,
            orientation: Orientation::North,
            footprint: 1,
        }
    }

    pub fn turn(&mut self) {
        match self.name {
            Name::I => self.turn_i(),
            Name::T => self.turn_t(),
            Name::S => self.turn_s(),
            Name::Z => self.turn_z(),
            Name::O => return,
            _ => return,
        }
    }

    pub fn simulate_a_turn(&mut self) -> [[u8; 2]; 4] {
        let mut substitute = self.clone();
        substitute.turn();
        substitute.blocks
    }

    pub fn turn_t(&mut self) {
        match self.orientation {
            Orientation::North => {
                self.blocks[1][0] -= 1;
                self.blocks[1][1] += 1;
                self.orientation = Orientation::West
            }
            Orientation::West => {
                self.blocks[2][0] -= 1;
                self.blocks[2][1] -= 1;
                self.orientation = Orientation::South
            }
            Orientation::South => {
                self.blocks[3][0] += 1;
                self.blocks[3][1] += 1;
                self.orientation = Orientation::East
            }
            Orientation::East => {
                self.blocks[1][0] += 1;
                self.blocks[1][1] += 1;
                self.orientation = Orientation::North
            }
        }
    }

    // I, footprint 2
    pub fn create_i() -> Self {
        Self {
            blocks: [[17, 5], [18, 5], [19, 5], [20, 5]],
            name: Name::I,
            orientation: Orientation::North,
            footprint: 2,
        }
    }
    pub fn turn_i(&mut self) {
        // check horizontality vs. verticality
        if self.blocks[0][0] != self.blocks[1][0] {
            // Move square one up and to the left
            self.blocks[0][0] += 1;
            self.blocks[0][1] -= 1;
            // Don't touche block two !
            // Move square three down and to the right
            self.blocks[2][0] -= 1;
            self.blocks[2][1] += 1;
            // move square four 2 steps down, 2 steps right
            self.blocks[3][0] -= 2;
            self.blocks[3][1] += 2;
        } else {
            self.blocks[0][0] -= 1;
            self.blocks[0][1] += 1;
            self.blocks[2][0] += 1;
            self.blocks[2][1] -= 1;
            self.blocks[3][0] += 2;
            self.blocks[3][1] -= 2;
        }
    }

    // S, footprint 3
    pub fn create_s() -> Self {
        Self {
            blocks: [[17, 3], [17, 4], [18, 4], [18, 5]],
            // the third is the center of rotation
            name: Name::S,
            orientation: Orientation::North,
            footprint: 3,
        }
    }

    pub fn turn_s(&mut self) {
        if self.orientation == Orientation::North {
            self.blocks[0][0] += 2;
            self.blocks[0][1] += 1;
            self.blocks[1][1] += 1;
            self.orientation = Orientation::East
        } else {
            self.blocks[0][0] -= 2;
            self.blocks[0][0] -= 1;
            self.blocks[1][1] -= 1;
            self.orientation = Orientation::North
        }
    }

    // Z, footprint 4
    pub fn create_z() -> Self {
        Self {
            blocks: [[17, 4], [17, 5], [18, 3], [18, 4]],
            // the last one is the center of rotation
            name: Name::Z,
            orientation: Orientation::North,
            footprint: 4,
        }
    }

    pub fn turn_z(&mut self) {
        if self.orientation == Orientation::North {
            self.blocks[2][0] += 1;
            self.blocks[2][1] += 2;
            self.blocks[1][1] += 1;
            self.orientation = Orientation::East
        } else {
            self.blocks[2][0] -= 1;
            self.blocks[2][1] -= 2;
            self.blocks[1][1] -= 1;
            self.orientation = Orientation::North
        }
    }

    // O, footprint 5
    pub fn create_o() -> Self {
        Self {
            blocks: [[17, 4], [17, 5], [18, 5], [18, 4]],
            name: Name::O,
            orientation: Orientation::North,
            footprint: 5,
        }
    }
}
