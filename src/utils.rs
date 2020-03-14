extern crate rand;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::io::{Read, Write};
// use std::{thread, time};
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::{clear, cursor};

pub struct Game<R, W: Write> {
    stdout: W,
    stdin: R,
    speed: u32,
    tetromino: Tetromino,
    // the "pile" is where the non-moving tetrominoes are piled
    pile: [Cell; 210],
    // the board is where the pile AND the moving tetrominoes are displayed
    board: [Cell; 210],
    score: u32,
}

#[derive(Debug, Copy)]
pub struct Tetromino {
    pub blocks: [u8; 4], // 4 coordinates
    pub name: Cell,
    pub spin: u8, // keeps track of the number of turns
}

impl Clone for Tetromino {
    fn clone(&self) -> Tetromino {
        *self
    }
}


impl<R: Read, W: Write> Game<R, W> {
    // Set a new empty game
    pub fn new(stdin: R, stdout: W) -> Game<R, RawTerminal<W>> {
        Game {
            stdout: stdout.into_raw_mode().unwrap(),
            stdin: stdin,
            speed: 800,
            tetromino: Tetromino {
                blocks: [174, 175, 176, 185],
                name: Cell::T,
                spin: 0,
            },
            pile: [Cell::Empty; 210],
            board: [Cell::Empty; 210], // fill the board with zeroes
            score: 0,
        }
    }

    pub fn run(&mut self) {

        let mut last_tick = std::time::SystemTime::now();

        loop {
            // takes user input and move the shape accordingly
            self.take_directions();

            // check if we reached time to tick again, then tick
            if last_tick.elapsed().unwrap().as_millis() as u32 >= self.speed {
                self.game_over(); // sets a game over if necessary
                self.tick();
                self.clear_full_rows();
                last_tick = std::time::SystemTime::now();
            }
        }
    }

    fn clear_full_rows(&mut self) {
        // Check all rows, as ranges
        for h in 0..17 {
            let row = (h * 10)..((h * 10) + 10);

            // If it contains no empty cell, replace it by its upstair neighbor
            // and recursively to the top. Set a new empty row at the top
            if !self.pile[row].contains(&Cell::Empty) {
                for i in (h * 10)..200 {
                    self.pile[i] = self.pile[i + 10]
                }
                for i in 200..210 {
                    self.pile[i] = Cell::Empty
                }
                // increase score and speed
                self.score += 1;
                self.speed -= 10;
            }
        }
    }

    // If the pile reaches up to the middle of the 17th row => game over
    fn game_over(&mut self) {
        for cell in 173..177 {
            if self.pile[cell as usize] != Cell::Empty {
                panic!("game over at score {}", self.score)
            }
        }
    }

    // tick the tetromino downward. If the move is not possible, freeze it and
    // call a new one
    fn tick(&mut self) {
        let coordinates = self.get_new_coordinates(Move::Down);
        let it_can_move = self.check_for_collisions(coordinates, Move::Down);
        if it_can_move {
            self.settle_the_move(coordinates);
        } else {
            // freeze the tetromino on the pile
            for coordinate in self.tetromino.blocks.iter() {
                self.pile[*coordinate as usize] = self.tetromino.name;
            }
            // call a new tetromino
            self.tetromino = Self::randow_new_tetromino();
        }
        self.display_the_board();
    }

    fn take_directions(&mut self) {
        // should be some nice error wrapping
        let mut b = [0];
        let mv: Move;
        self.stdin.read(&mut b).unwrap();
        match b[0] {
            b'i' => mv = Move::Turn,
            b'j' => mv = Move::Left,
            b'k' => mv = Move::Down,
            b'l' => mv = Move::Right,
            b'q' => panic!("c'est la panique !"),
            _ => return, // should be some nice error handling
        }

        let coordinates = self.get_new_coordinates(mv.clone());
        let it_can_move = self.check_for_collisions(coordinates, mv);
        if it_can_move {
            self.settle_the_move(coordinates);
        } else {
            return;
        }
        self.stdout.flush().unwrap();
    }

    fn get_new_coordinates(&mut self, mv: Move) -> [u8; 4] {
        let coordinates: [u8; 4];
        match mv {
            Move::Left => coordinates = self.push_left(),
            Move::Right => coordinates = self.push_right(),
            Move::Down => coordinates = self.push_down(),
            // warning: the turn() function here will change the spin field of the
            // Tetromino struct. If the move doesn't occur, the check_for_collision()
            // function will reset the spin
            Move::Turn => coordinates = self.turn(),
        }
        coordinates
    }

    fn check_for_collisions(&mut self, coordinates: [u8; 4], mv: Move) -> bool {
        let mut no_collision = true;

        for i in coordinates.iter() {
            // Check for collision with the wall (overlapping the % 10 frontier)
            if i % 10 == 0 {
                for i in coordinates.iter() {
                    if (i + 1) % 10 == 0 {
                        no_collision = false;
                    }
                }
            }

            // check for collisions with the pile
            if self.pile[*i as usize] != Cell::Empty {
                no_collision = false;
            }

            // check for collisions with the bottom
            if i < &10 {
                no_collision = false;
            }
        }

        // since a vertical I tetromino doesn't overlapp the wall border (% 10),
        // I have the sad duty to implement an extra rule
        if self.tetromino.name == Cell::I {
            if (mv == Move::Left && (coordinates[0] + 1) % 10 == 0)
                || (mv == Move::Right && (coordinates[0]) % 10 == 0)
            {
                no_collision = false;
            }

        }

        // in case of a turn collision, set the tetromino's spin back to what it was
        if !no_collision && mv == Move::Turn {
            self.tetromino.spin -= 1;
        }
        // Return the boolean
        no_collision
    }

    // This set the new position of the tetromino
    fn settle_the_move(&mut self, coordinates: [u8; 4]) {
        self.tetromino.blocks = coordinates;
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

        // the bottom line is empty for logic purposes, suppress it
        let mut board_to_draw = [Cell::Empty; 200];
        for i in 0..200 {
            board_to_draw[i] = self.board[i + 10];
        }

        for line in board_to_draw.chunks(10).rev() {
            // display each lines two times
            for _i in 0..2 {
                self.stdout.write(b"|").unwrap();
                for &cell in line.iter() {
                    let symbol = match cell {
                        Cell::Empty => b"   ",
                        Cell::T => b"TTT",
                        Cell::I => b"III",
                        Cell::S => b"SSS",
                        Cell::Z => b"ZZZ",
                        Cell::O => b"OOO",
                        Cell::L => b"LLL",
                        Cell::J => b"JJJ",
                        _ => b"XXX",
                    };
                    self.stdout.write(symbol).unwrap();
                }
                self.stdout.write(b"|\n\r").unwrap();
            }
        }
        // bottom wall
        for _n in 0..32 {
            self.stdout.write(b"-").unwrap();
        }
        self.stdout.flush().unwrap();
    }

    fn randow_new_tetromino() -> Tetromino {
        let name: Cell = rand::random(); // where does this random() come from ?
        Tetromino {
            blocks: match name {
                Cell::T => [174, 175, 176, 185],
                Cell::L => [184, 174, 194, 175],
                Cell::J => [185, 175, 195, 174],
                Cell::I => [175, 185, 195, 205],
                Cell::S => [174, 175, 185, 186],
                Cell::Z => [175, 176, 184, 185],
                Cell::O => [174, 175, 185, 184],
                Cell::Empty => panic!("problème de création aléatoire"),
            },
            name,
            spin: 0,
        }
    }

    // The push functions provide the tetromino's new position in case of a move
    // Those coordinates will be tested for possible collisions
    fn push_down(&self) -> [u8; 4] {
        let mut coordinates: [u8; 4] = self.tetromino.blocks.clone();
        for i in 0..4 {
            coordinates[i as usize] -= 10;
        }
        coordinates
    }
    fn push_left(&self) -> [u8; 4] {
        let mut coordinates: [u8; 4] = self.tetromino.blocks.clone();
        for i in 0..4 {
            coordinates[i as usize] -= 1;
        }
        coordinates
    }
    fn push_right(&self) -> [u8; 4] {
        let mut coordinates: [u8; 4] = self.tetromino.blocks.clone();
        for i in 0..4 {
            coordinates[i as usize] += 1;
        }
        coordinates
    }
    fn turn(&mut self) -> [u8; 4] {
        let mut coordinates: [u8; 4] = self.tetromino.blocks.clone();

        match self.tetromino.spin {
            0 => {
                match self.tetromino.name {
                    Cell::T => coordinates[0] -= 9,
                    Cell::L => {
                        coordinates[1] += 11;
                        coordinates[2] -= 11;
                        coordinates[3] -= 2;
                    }
                    Cell::J => {
                        coordinates[1] += 11;
                        coordinates[2] -= 11;
                        coordinates[3] += 20;
                    }
                    Cell::I => {
                        coordinates[0] += 9;
                        coordinates[2] -= 9;
                        coordinates[3] -= 18;
                    }
                    Cell::S => {
                        coordinates[0] += 21;
                        coordinates[1] += 1;
                    }
                    Cell::Z => {
                        coordinates[1] += 20;
                        coordinates[2] += 2;
                    }
                    Cell::O => return coordinates,
                    _ => return coordinates,
                }
                self.tetromino.spin += 1;
            }
            1 => match self.tetromino.name {
                Cell::T => {
                    coordinates[3] -= 11;
                    self.tetromino.spin += 1;
                }
                Cell::L => {
                    coordinates[1] -= 11;
                    coordinates[2] += 11;
                    coordinates[3] += 20;
                    self.tetromino.spin += 1;
                }
                Cell::J => {
                    coordinates[1] -= 11;
                    coordinates[2] += 11;
                    coordinates[3] += 2;
                    self.tetromino.spin += 1;
                }
                Cell::I => {
                    coordinates[0] -= 9;
                    coordinates[2] += 9;
                    coordinates[3] += 18;
                    self.tetromino.spin -= 1; // return to the first spin
                }
                Cell::S => {
                    coordinates[0] -= 21;
                    coordinates[1] -= 1;
                    self.tetromino.spin -= 1;
                }
                Cell::Z => {
                    coordinates[1] -= 20;
                    coordinates[2] -= 2;
                    self.tetromino.spin -= 1;
                }
                _ => return coordinates,
            },
            2 => {
                match self.tetromino.name {
                    Cell::T => coordinates[2] += 9,
                    Cell::L => {
                        coordinates[1] += 11;
                        coordinates[2] -= 11;
                        coordinates[3] += 2;
                    }
                    Cell::J => {
                        coordinates[1] += 11;
                        coordinates[2] -= 11;
                        coordinates[3] -= 20;
                    }
                    _ => return coordinates,
                };
                self.tetromino.spin += 1;
            }
            3 => {
                match self.tetromino.name {
                    Cell::T => {
                        coordinates[0] += 9;
                        coordinates[3] += 11;
                        coordinates[2] -= 9;
                    }
                    Cell::L => {
                        coordinates[1] -= 11;
                        coordinates[2] += 11;
                        coordinates[3] -= 20;
                    }
                    Cell::J => {
                        coordinates[1] -= 11;
                        coordinates[2] += 11;
                        coordinates[3] -= 2;
                    }
                    _ => return coordinates,
                };
                self.tetromino.spin -= 3;
            }
            _ => return coordinates,
        }
        coordinates
    }
}

#[derive(PartialEq, Clone)]
enum Move {
    Left,
    Right,
    Down,
    Turn,
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

// randomization stuff around chosing the next tetromino
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
