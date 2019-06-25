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
    pile: [Cell; 210],
    // the board is where the pile AND the moving tetrominoes are displayed
    board: [Cell; 210],
}

#[derive(Debug, Clone)]
pub struct Tetromino {
    pub blocks: [u8; 4], // 4 coordinates
    pub name: Cell,
    pub orientation: Orientation,
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
                name: Cell::T,
                orientation: Orientation::North,
                can_move_down: true,
            },
            pile: [Cell::Empty; 210],
            board: [Cell::Empty; 210], // fill the board with zeroes
        };
        new_game
    }

    pub fn run(&mut self) {
        loop {
            let mv = self.take_directions();
            self.move_or_not(mv);

            self.display_the_board();
            self.tick();
            self.clear_full_rows();
            self.game_over();
            thread::sleep(time::Duration::from_millis(self.speed));
        }
    }

    fn clear_full_rows(&mut self) {
        for h in 0..17 {
            let range = (h * 10)..((h *10) + 10);
            if !self.pile[range].contains(&Cell::Empty) {
                for i in (h * 10)..200 {
                    self.pile[i] = self.pile[i +10]
                }
                for i in 200..210 {
                    self.pile[i] = Cell::Empty
                }
            }
        }
    }

    fn game_over(&mut self) {
        for cell in 170..180 {
            if self.pile[cell as usize] != Cell::Empty {
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
                if self.pile[*coordinate as usize - 10] != Cell::Empty {
                    self.tetromino.can_move_down = false;
                }
            }
        }

        if self.tetromino.can_move_down {
            self.move_or_not(Move::Down);
        } else {
            // this writes the tetromino footprint on the "pile"
            for i in self.tetromino.blocks.iter() {
                self.pile[*i as usize] = self.tetromino.name;
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
            if self.pile[*coordinate as usize] != Cell::Empty {
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
        // Draw the pile on the board (this erases the previously appearing tetromino)
        self.board = self.pile;

        // Draw the tetromino on the board
        for i in self.tetromino.blocks.iter() {
            self.board[*i as usize] = self.tetromino.name;
        }

        // Display the whole damn thing
        write!(self.stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();

        for line in self.board.chunks(10).rev() {
            self.stdout.write(b"|").unwrap();
            for &cell in line.iter() {
                let symbol = match cell {
                    Cell::Empty => b" ",
                    Cell::T => b"T",
                    Cell::I => b"I",
                    Cell::S => b"S",
                    Cell::Z => b"Z",
                    Cell::O => b"O",
                    Cell::L => b"L",
                    Cell::J => b"J",
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


    fn randow_new_tetromino() -> Tetromino {
        let name: Cell = rand::random(); // where does this random() come from ?
        let new_tetromino: Tetromino;

        let new_tetromino = Tetromino {
            blocks: match name {
                Cell::I => [175, 185, 195, 205],
                Cell::T => [174, 175, 176, 185],
                Cell::S => [174, 175, 185, 186],
                Cell::Z => [175, 176, 184, 185],
                Cell::O => [174, 175, 185, 184],
                Cell::L => [184, 174, 194, 175],
                Cell::J => [185, 175, 195, 174],
                Cell::Empty => panic!("problème de création aléatoire")
                // "O" => create_o(),
            },
            name: name,
            orientation: Orientation::North,
            can_move_down: true,
        };
        new_tetromino
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
            Cell::T => self.turn_t(),
            Cell::I => self.turn_i(),
            Cell::S => self.turn_s(),
            Cell::Z => self.turn_z(),
            Cell::O => self.turn_o(),
            Cell::L => self.turn_l(),
            Cell::J => self.turn_j(),
            Cell::Empty => panic!("if this panics we really have a problem")
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
    fn turn_o(&mut self) {
        return;
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

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Orientation {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Cell {
    // Here are the footprints:
    T, // 1
    I, // 2
    S, // 3
    Z, // 4
    O, // 5
    L, // 6
    J, // 7
    Empty,
}

// Some whirly-dirly randomization stuff around chosing the next tetromino
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
