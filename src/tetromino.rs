use crate::game::Move;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug, Copy)]
pub struct Tetromino {
    pub coordinates: [u8; 4],
    pub name: Cell,
    pub spin: u8,
}

impl Tetromino {
    pub fn randow_new() -> Self {
        let name: Cell = rand::random();
        let new_tetromino = Self {
            coordinates: match name {
                Cell::T => [174, 175, 176, 185],
                Cell::L => [184, 174, 194, 175],
                Cell::J => [185, 175, 195, 174],
                Cell::I => [175, 185, 195, 205],
                Cell::S => [174, 175, 185, 186],
                Cell::Z => [175, 176, 184, 185],
                Cell::O => [174, 175, 185, 184],
                Cell::Empty => panic!("The random_new tetromino function messed up."),
            },
            name,
            spin: 0,
        };
        new_tetromino
    }

    pub fn move_it(&mut self, direction: &Move) {
        match direction {
            Move::Left => {
                for i in 0..4 as usize {
                    self.coordinates[i] -= 1;
                }
            }
            Move::Right => {
                for i in 0..4 as usize {
                    self.coordinates[i] += 1;
                }
            }
            Move::Down => {
                for i in 0..4 as usize {
                    self.coordinates[i] -= 10;
                }
            }
            Move::Turn => {
                match self.spin {
                    0 => {
                        match self.name {
                            Cell::T => self.coordinates[0] -= 9,
                            Cell::L => {
                                self.coordinates[1] += 11;
                                self.coordinates[2] -= 11;
                                self.coordinates[3] -= 2;
                            }
                            Cell::J => {
                                self.coordinates[1] += 11;
                                self.coordinates[2] -= 11;
                                self.coordinates[3] += 20;
                            }
                            Cell::I => {
                                self.coordinates[0] += 9;
                                self.coordinates[2] -= 9;
                                self.coordinates[3] -= 18;
                            }
                            Cell::S => {
                                self.coordinates[0] += 21;
                                self.coordinates[1] += 1;
                            }
                            Cell::Z => {
                                self.coordinates[1] += 20;
                                self.coordinates[2] += 2;
                            }
                            _ => return,
                        }
                        self.spin += 1;
                    }
                    1 => match self.name {
                        Cell::T => {
                            self.coordinates[3] -= 11;
                            self.spin += 1;
                        }
                        Cell::L => {
                            self.coordinates[1] -= 11;
                            self.coordinates[2] += 11;
                            self.coordinates[3] += 20;
                            self.spin += 1;
                        }
                        Cell::J => {
                            self.coordinates[1] -= 11;
                            self.coordinates[2] += 11;
                            self.coordinates[3] += 2;
                            self.spin += 1;
                        }
                        Cell::I => {
                            self.coordinates[0] -= 9;
                            self.coordinates[2] += 9;
                            self.coordinates[3] += 18;
                            self.spin -= 1; // return to the first spin
                        }
                        Cell::S => {
                            self.coordinates[0] -= 21;
                            self.coordinates[1] -= 1;
                            self.spin -= 1;
                        }
                        Cell::Z => {
                            self.coordinates[1] -= 20;
                            self.coordinates[2] -= 2;
                            self.spin -= 1;
                        }
                        _ => return,
                    },
                    2 => {
                        match self.name {
                            Cell::T => self.coordinates[2] += 9,
                            Cell::L => {
                                self.coordinates[1] += 11;
                                self.coordinates[2] -= 11;
                                self.coordinates[3] += 2;
                            }
                            Cell::J => {
                                self.coordinates[1] += 11;
                                self.coordinates[2] -= 11;
                                self.coordinates[3] -= 20;
                            }
                            _ => return,
                        };
                        self.spin += 1;
                    }
                    3 => {
                        match self.name {
                            Cell::T => {
                                self.coordinates[0] += 9;
                                self.coordinates[3] += 11;
                                self.coordinates[2] -= 9;
                            }
                            Cell::L => {
                                self.coordinates[1] -= 11;
                                self.coordinates[2] += 11;
                                self.coordinates[3] -= 20;
                            }
                            Cell::J => {
                                self.coordinates[1] -= 11;
                                self.coordinates[2] += 11;
                                self.coordinates[3] -= 2;
                            }
                            _ => return,
                        };
                        self.spin -= 3;
                    }
                    _ => return,
                }
            }
            Move::None => return,
        }
    }
}

impl Clone for Tetromino {
    fn clone(&self) -> Tetromino {
        *self
    }
}

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
