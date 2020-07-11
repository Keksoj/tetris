extern crate rand;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::io::{Read, Write};
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::{clear, cursor};

#[derive(Debug, Copy)]
pub struct Tetromino {
    pub coordinates: [u8; 4],
    pub name: Cell,
    pub spin: u8,
}
impl Tetromino {
    fn randow_new() -> Self {
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

    fn move_it(&mut self, direction: &Move) {
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

pub struct GameBuilder<R, W: Write> {
    stdout: W,
    stdin: R,
    speed: u32,
    tetromino: Tetromino,
    direction: Move,
    next_move_tetromino: Tetromino,
    move_is_possible: bool,
    stack: [Cell; 210],
    display_board: [Cell; 210],
    score: u32,
}

impl<R: Read, W: Write> GameBuilder<R, W> {
    pub fn new_default_game(stdin: R, stdout: W) -> GameBuilder<R, RawTerminal<W>> {
        GameBuilder {
            stdout: stdout.into_raw_mode().unwrap(),
            stdin: stdin,
            speed: 800,
            tetromino: Tetromino::randow_new(),
            direction: Move::None,
            next_move_tetromino: Tetromino::randow_new(),
            move_is_possible: true,
            stack: [Cell::Empty; 210],
            display_board: [Cell::Empty; 210],
            score: 0,
        }
    }
    pub fn with_initial_speed(mut self, tick: u32) -> Self {
        self.speed = tick;
        self
    }
    pub fn finish(self) -> Game<R, W> {
        Game {
            stdout: self.stdout,
            stdin: self.stdin,
            speed: self.speed,
            tetromino: self.tetromino,
            direction: self.direction,
            next_move_tetromino: self.next_move_tetromino,
            move_is_possible: self.move_is_possible,
            stack: self.stack,
            display_board: self.display_board,
            score: self.score,
        }
    }
}

pub struct Game<R, W: Write> {
    stdout: W,
    stdin: R,
    speed: u32,
    tetromino: Tetromino,
    direction: Move,
    next_move_tetromino: Tetromino,
    move_is_possible: bool,
    stack: [Cell; 210],
    display_board: [Cell; 210],
    score: u32,
}

impl<R: Read, W: Write> Game<R, W> {
    pub fn run(&mut self) {
        let mut last_tick = std::time::SystemTime::now();

        loop {
            self.take_directions();

            if last_tick.elapsed().unwrap().as_millis() as u32 >= self.speed {
                self.check_for_game_over();
                self.tick();
                self.clear_full_rows();
                last_tick = std::time::SystemTime::now();
            }
        }
    }

    fn clear_full_rows(&mut self) {
        for hight in 0..17 {
            let row = (hight * 10)..((hight * 10) + 10);

            if !self.stack[row].contains(&Cell::Empty) {
                for i in (hight * 10)..200 {
                    self.stack[i] = self.stack[i + 10]
                }
                for i in 200..210 {
                    self.stack[i] = Cell::Empty
                }
                self.score += 1;
                self.speed -= 10;
            }
        }
    }

    fn check_for_game_over(&mut self) {
        for cell in 173..177 {
            if self.stack[cell as usize] != Cell::Empty {
                panic!("game over at score {}", self.score)
            }
        }
    }

    fn tick(&mut self) {
        self.direction = Move::Down;
        self.compute_the_next_move();
        self.check_for_collisions();
        if self.move_is_possible {
            self.settle_the_move();
        } else {
            self.freeze_and_next();
        }
        self.display_the_board();
    }
    
    fn freeze_and_next(&mut self) {
        for coordinate in self.tetromino.coordinates.iter() {
            self.stack[*coordinate as usize] = self.tetromino.name;
        }
        self.tetromino = Tetromino::randow_new();
        self.move_is_possible = true;
        // self.generate_randow_new_tetromino();
    }

    fn take_directions(&mut self) {
        // should be some nice error handling
        let mut b = [0];
        self.stdin.read(&mut b).unwrap();
        match b[0] {
            b'i' => self.direction = Move::Turn,
            b'j' => self.direction = Move::Left,
            b'k' => self.direction = Move::Down,
            b'l' => self.direction = Move::Right,
            b'q' => panic!("c'est la panique !"),
            _ => self.direction = Move::None,
        }
        if self.direction != Move::None {
            self.compute_the_next_move();
            self.check_for_collisions();
            if self.move_is_possible {
                self.settle_the_move();
            }
        }
    }

    fn compute_the_next_move(&mut self) {
        self.next_move_tetromino = self.tetromino;
        self.next_move_tetromino.move_it(&self.direction);
    }

    fn settle_the_move(&mut self) {
        self.tetromino = self.next_move_tetromino;
        self.display_the_board();
    }

    fn check_for_collisions(&mut self) {
        self.move_is_possible = true;
        for i in self.next_move_tetromino.coordinates.iter() {
            // wall collisions (overlapping the % 10 frontier)
            if i % 10 == 0 {
                for i in self.next_move_tetromino.coordinates.iter() {
                    if (i + 1) % 10 == 0 {
                        self.move_is_possible = false;
                    }
                }
            }
            // stack collisions
            if self.stack[*i as usize] != Cell::Empty {
                self.move_is_possible = false;
            }
            // floor collisions
            if i < &10 {
                self.move_is_possible = false;
            }
        }
        // wall collision for vertical I
        if self.tetromino.name == Cell::I {
            if (self.direction == Move::Left
                && (self.next_move_tetromino.coordinates[0] + 1) % 10 == 0)
                || (self.direction == Move::Right
                    && (self.next_move_tetromino.coordinates[0]) % 10 == 0)
            {
                self.move_is_possible = false;
            }
        }
    }

    fn display_the_board(&mut self) {
        // Draw the stack on the display_board (this erases the previously appearing tetromino)
        self.display_board = self.stack;

        // Draw the tetromino on the display_board
        for i in self.tetromino.coordinates.iter() {
            self.display_board[*i as usize] = self.tetromino.name;
        }

        // Display the whole damn thing
        write!(self.stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();

        // the bottom line is empty for logic purposes, suppress it
        let mut board_to_draw = [Cell::Empty; 200];
        for i in 0..200 {
            board_to_draw[i] = self.display_board[i + 10];
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
                    };
                    self.stdout.write(symbol).unwrap();
                }
                self.stdout.write(b"|\n\r").unwrap();
            }
        }
        for _n in 0..32 {
            self.stdout.write(b"-").unwrap();
        }
        self.stdout.flush().unwrap();
    }
}

#[derive(PartialEq, Clone)]
enum Move {
    Left,
    Right,
    Down,
    Turn,
    None,
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
