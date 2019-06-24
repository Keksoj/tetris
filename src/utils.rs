extern crate rand;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::io::{Read, Write};
use std::{thread, time};
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::{clear, cursor};

// #[derive(Debug)]
pub struct Game<R, W: Write> {
    stdout: W,
    stdin: R,
    speed: u64,
    tetromino: Tetromino,
    // the "pile" is where the non-moving tetrominoes are piled
    pile: [u8; 210],
    // the board is where the pile AND the moving tetrominoes are displayed
    board: [u8; 210],
}

#[derive(Debug, Clone)]
pub struct Tetromino {
    pub blocks: [u8; 4], // 4 coordinates
    pub name: Name,
    pub orientation: Orientation,
    pub footprint: u8,
    pub can_move_down: bool,
}

impl<R: Read, W: Write> Game<R, W> {
    // Set a new empty game
    pub fn new(stdin: R, stdout: W) -> Game<R, RawTerminal<W>> {
        let new_game = Game {
            stdout: stdout.into_raw_mode().unwrap(),
            stdin: stdin,
            speed: 700,
            tetromino: Tetromino {
                blocks: [174, 175, 176, 185],
                name: Name::T,
                orientation: Orientation::North,
                footprint: 1,
                can_move_down: true,
            },
            pile: [0; 210],
            board: [0; 210], // fill the board with zeroes
        };
        new_game
    }

    fn push_down(&mut self) {
        for i in 0..4 {
            self.tetromino.blocks[i] -= 10;
        }
    }
    fn push_up(&mut self) {
        for i in 0..4 {
            self.tetromino.blocks[i] += 10;
        }
    }
    fn push_left(&mut self) {
        for i in 0..4 {
            self.tetromino.blocks[i] -= 1
        }
    }
    fn push_right(&mut self) {
        for i in 0..4 {
            self.tetromino.blocks[i] += 1;
        }
    }
    fn turn(&mut self) {
        match self.tetromino.name {
            Name::T => self.turn_t(),
            Name::I => self.turn_i(),
            Name::S => self.turn_s(),
            Name::Z => self.turn_z(),
            Name::O => self.turn_o(),
            Name::L => self.turn_l(),
            Name::J => self.turn_j(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let mv = self.take_directions();
            self.move_or_not(mv);

            self.display_the_board();
            self.tick();
            self.clear_full_row();
            self.game_over();
            thread::sleep(time::Duration::from_millis(1000));
        }
    }

    fn clear_full_row(&mut self) {
        for line in self.pile.chunks(10) {
            if !line.contains(&0) {
                panic!("full row")
            }
        }
    }

    fn game_over(&mut self) {
        for cell in 170..180 {
            if self.pile[cell as usize] != 0 {
                panic!("game over")
            }
        }
    }

    fn tick(&mut self) {
        // check for bottom collision
        for i in 0..4 {
            if self.tetromino.blocks[i as usize] < 10 {
                self.tetromino.can_move_down = false;
            }
        }

        // check for downward collision with the pile
        if self.tetromino.can_move_down {
            for coordinate in self.tetromino.blocks.iter() {
                if self.pile[*coordinate as usize - 10] != 0 {
                    self.tetromino.can_move_down = false;
                }
            }
        }

        if self.tetromino.can_move_down {
            self.move_or_not(Move::Down);
        } else {
            // this writes the tetromino footprint on the "pile"
            for i in self.tetromino.blocks.iter() {
                self.pile[*i as usize] = self.tetromino.footprint;
            }
            // This calls the new tetromino
            self.tetromino = Self::randow_new_tetromino();
        }
    }

    fn take_directions(&mut self) -> Move {
        let mv: Move;

        // should be some nice error wrapping
        let mut b = [0];
        self.stdin.read(&mut b).unwrap();
        match b[0] {
            b'i' => mv = Move::Turn,
            b'j' => mv = Move::Left,
            b'k' => mv = Move::Down,
            b'l' => mv = Move::Right,
            b'q' => panic!("c'est la panique !"),
            _ => mv = Move::Nothing, // should be some nice error handling
        }
        // I stil have to understand what this does.
        self.stdout.flush().unwrap();
        mv
    }

    fn move_or_not(&mut self, mv: Move) {
        // First, check for bottom collision, or we'll have subtract with overflow
        for i in 0..4 {
            if mv == Move::Down && self.tetromino.blocks[i as usize] < 10 {
                return;
            }
        }
        // Performs the move
        match mv {
            Move::Left => self.push_left(),
            Move::Right => self.push_right(),
            Move::Down => self.push_down(),
            Move::Turn => self.turn(),
            Move::Nothing => return,
        }

        let mut it_collides = false;
        // move left / check for collisions with the left wall
        if mv == Move::Left {
            for i in 0..4 {
                if (self.tetromino.blocks[i as usize] + 1) % 10 == 0 {
                    it_collides = true;
                }
            }
        }
        // move right / check for collisions with the right wall
        if mv == Move::Right {
            for i in 0..4 {
                if self.tetromino.blocks[i as usize] % 10 == 0 {
                    it_collides = true;
                }
            }
        }
        // turn / check overlapping of the % 10 frontier
        if mv == Move::Turn {
            for i in 0..4 {
                if self.tetromino.blocks[i as usize] % 10 == 0 {
                    for i in 0..4 {
                        if (self.tetromino.blocks[i as usize] + 1) % 10 == 0 {
                            it_collides = true;
                        }
                    }
                }
            }
        }
        // check for collisions with the pile
        for coordinate in self.tetromino.blocks.iter() {
            if self.pile[*coordinate as usize] != 0 {
                it_collides = true;
            }
        }
        // undoes the mv in case of a collision
        if it_collides {
            match mv {
                Move::Left => self.push_right(),
                Move::Right => self.push_left(),
                Move::Down => self.push_up(),
                Move::Turn => {
                    self.turn();
                    self.turn();
                    self.turn();
                }
                Move::Nothing => return,
            }
        }
        self.display_the_board();
    }

    fn display_the_board(&mut self) {
        // Draw the pile (this erases the previously appearing tetromino)
        self.board = self.pile;

        // Draw the tetromino
        for i in self.tetromino.blocks.iter() {
            self.board[*i as usize] = self.tetromino.footprint;
        }

        // Display the whole damn thing
        write!(self.stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();

        for line in self.board.chunks(10).rev() {
            self.stdout.write(b"|").unwrap();
            for &cell in line.iter() {
                let symbol = match cell {
                    0 => b" ",
                    1 => b"T",
                    2 => b"I",
                    3 => b"S",
                    4 => b"Z",
                    5 => b"O",
                    6 => b"L",
                    7 => b"J",
                    _ => b"X",
                };
                self.stdout.write(symbol).unwrap();
            }
            self.stdout.write(b"|\n\r").unwrap();
        }
        // bottom wall
        for _n in 0..12 {
            self.stdout.write(b"-").unwrap();
        }
        self.stdout.flush().unwrap();
        // write!(self.stdout, "{}", cursor::Hide).unwrap();
    }

    // Here come the Tetromino functions
    fn randow_new_tetromino() -> Tetromino {
        let name: Name = rand::random(); // where does this random() come from ?
        let new_tetromino: Tetromino;
        match name {
            Name::I => new_tetromino = Self::create_i(),
            Name::T => new_tetromino = Self::create_t(),
            Name::S => new_tetromino = Self::create_s(),
            Name::Z => new_tetromino = Self::create_z(),
            Name::O => new_tetromino = Self::create_o(),
            Name::L => new_tetromino = Self::create_l(),
            Name::J => new_tetromino = Self::create_j(),
            // "O" => create_o(),
        }
        new_tetromino
    }

    // T, footprint 1
    fn create_t() -> Tetromino {
        Tetromino {
            // second item won't mv
            blocks: [174, 175, 176, 185],
            name: Name::T,
            orientation: Orientation::North,
            footprint: 1,
            can_move_down: true,
        }
    }
    fn turn_t(&mut self) {
        match self.tetromino.orientation {
            Orientation::North => {
                self.tetromino.blocks[0] -= 9;
                self.tetromino.orientation = Orientation::East
            }
            Orientation::East => {
                self.tetromino.blocks[3] -= 11;
                self.tetromino.orientation = Orientation::South
            }
            Orientation::South => {
                self.tetromino.blocks[2] += 9;
                self.tetromino.orientation = Orientation::West
            }
            Orientation::West => {
                self.tetromino.blocks[0] += 9;
                self.tetromino.blocks[3] += 11;
                self.tetromino.blocks[2] -= 9;
                self.tetromino.orientation = Orientation::North
            }
        }
    }

    // I, footprint 2
    fn create_i() -> Tetromino {
        Tetromino {
            blocks: [175, 185, 195, 205],
            name: Name::I,
            orientation: Orientation::North,
            footprint: 2,
            can_move_down: true,
        }
    }
    fn turn_i(&mut self) {
        // check for verticality
        if self.tetromino.blocks[0] == self.tetromino.blocks[1] - 10 {
            self.tetromino.blocks[0] += 9;
            self.tetromino.blocks[2] -= 9;
            self.tetromino.blocks[3] -= 18;
        } else {
            self.tetromino.blocks[0] -= 9;
            self.tetromino.blocks[2] += 9;
            self.tetromino.blocks[3] += 18;
        }
    }

    // S, footprint 3
    fn create_s() -> Tetromino {
        Tetromino {
            blocks: [174, 175, 185, 186],
            // the third is the center of rotation
            name: Name::S,
            orientation: Orientation::North,
            footprint: 3,
            can_move_down: true,
        }
    }

    pub fn turn_s(&mut self) {
        // check for collisions
        if self.tetromino.orientation == Orientation::North {
            self.tetromino.blocks[0] += 21;
            self.tetromino.blocks[1] += 1;
            self.tetromino.orientation = Orientation::East
        } else {
            self.tetromino.blocks[0] -= 21;
            self.tetromino.blocks[1] -= 1;
            self.tetromino.orientation = Orientation::North
        }
    }

    // Z, footprint 4
    fn create_z() -> Tetromino {
        Tetromino {
            blocks: [175, 176, 184, 185],
            // the last one is the center of rotation
            name: Name::Z,
            orientation: Orientation::North,
            footprint: 4,
            can_move_down: true,
        }
    }
    fn turn_z(&mut self) {
        if self.tetromino.orientation == Orientation::North {
            self.tetromino.blocks[1] += 20;
            self.tetromino.blocks[2] += 2;
            self.tetromino.orientation = Orientation::East
        } else {
            self.tetromino.blocks[1] -= 20;
            self.tetromino.blocks[2] -= 2;
            self.tetromino.orientation = Orientation::North
        }
    }
    // O, footprint 5
    fn create_o() -> Tetromino {
        Tetromino {
            blocks: [174, 175, 185, 184],
            name: Name::O,
            orientation: Orientation::North,
            footprint: 5,
            can_move_down: true,
        }
    }
    fn turn_o(&mut self) {
        return;
    }
    // L, footprint 6
    fn create_l() -> Tetromino {
        Tetromino {
            blocks: [184, 174, 194, 175],
            name: Name::L,
            orientation: Orientation::North,
            footprint: 6,
            can_move_down: true,
        }
    }
    fn turn_l(&mut self) {
        if self.tetromino.orientation == Orientation::North {
            self.tetromino.blocks[1] += 11;
            self.tetromino.blocks[2] -= 11;
            self.tetromino.blocks[3] -= 2;
            self.tetromino.orientation = Orientation::East
        } else if self.tetromino.orientation == Orientation::East {
            self.tetromino.blocks[1] -= 11;
            self.tetromino.blocks[2] += 11;
            self.tetromino.blocks[3] += 20;
            self.tetromino.orientation = Orientation::South
        } else if self.tetromino.orientation == Orientation::South {
            self.tetromino.blocks[1] += 11;
            self.tetromino.blocks[2] -= 11;
            self.tetromino.blocks[3] += 2;
            self.tetromino.orientation = Orientation::West
        } else if self.tetromino.orientation == Orientation::West {
            self.tetromino.blocks[1] -= 11;
            self.tetromino.blocks[2] += 11;
            self.tetromino.blocks[3] -= 20;
            self.tetromino.orientation = Orientation::North;
        }
    }
    // J, footprint 7
    fn create_j() -> Tetromino {
        Tetromino {
            blocks: [185, 175, 195, 174],
            name: Name::J,
            orientation: Orientation::North,
            footprint: 6,
            can_move_down: true,
        }
    }
    fn turn_j(&mut self) {
        if self.tetromino.orientation == Orientation::North {
            self.tetromino.blocks[1] += 11;
            self.tetromino.blocks[2] -= 11;
            self.tetromino.blocks[3] += 20;
            self.tetromino.orientation = Orientation::East
        } else if self.tetromino.orientation == Orientation::East {
            self.tetromino.blocks[1] -= 11;
            self.tetromino.blocks[2] += 11;
            self.tetromino.blocks[3] += 2;
            self.tetromino.orientation = Orientation::South
        } else if self.tetromino.orientation == Orientation::South {
            self.tetromino.blocks[1] += 11;
            self.tetromino.blocks[2] -= 11;
            self.tetromino.blocks[3] -= 20;
            self.tetromino.orientation = Orientation::West
        } else if self.tetromino.orientation == Orientation::West {
            self.tetromino.blocks[1] -= 11;
            self.tetromino.blocks[2] += 11;
            self.tetromino.blocks[3] -= 2;
            self.tetromino.orientation = Orientation::North;
        }
    }
}

#[derive(PartialEq)]
enum Move {
    Left,
    Right,
    Down,
    Turn,
    Nothing,
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
