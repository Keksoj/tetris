extern crate rand;

use std::io::{Read, Write};
use std::time::SystemTime;
use termion::{
    clear, cursor,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    AsyncReader,
};

use crate::cell::Cell;
use crate::tetromino::Tetromino;

pub struct Game<W: Write> {
    stdout: W,
    stdin: AsyncReader,
    tick_interval: u32,
    tetromino: Tetromino,
    direction: Move,
    next_move_tetromino: Tetromino,
    move_is_possible: bool,
    stack: [Cell; 210],
    display_board: [Cell; 210],
    score: u32,
    debug_message: String,
}

impl<W: Write> Game<W> {
    pub fn new(stdin: AsyncReader, stdout: W, tick_interval: u32) -> Game<RawTerminal<W>> {
        Game {
            stdout: stdout.into_raw_mode().unwrap(),
            stdin,
            tick_interval,
            tetromino: Tetromino::randow_new(),
            direction: Move::None,
            next_move_tetromino: Tetromino::randow_new(),
            move_is_possible: true,
            stack: [Cell::Empty; 210],
            display_board: [Cell::Empty; 210],
            score: 0,
            debug_message: String::new(),
        }
    }

    pub fn run(&mut self) {
        let mut last_tick = SystemTime::now();
        loop {
            self.take_one_direction();
            if last_tick.elapsed().unwrap().as_millis() as u32 >= self.tick_interval {
                self.check_for_game_over();
                self.tick();
                self.clear_full_rows();
                last_tick = SystemTime::now();
            }
        }
    }

    fn take_one_direction(&mut self) {
        let stdin = std::io::stdin();
        let mut keys = stdin.keys();
        self.debug("Taking directions");

        let first_key = keys.next().unwrap();
        self.debug(format!("Pressing key {:?}", first_key));
        match first_key.unwrap() {
            Key::Left => self.direction = Move::Left,
            Key::Right => self.direction = Move::Right,
            Key::Up => self.direction = Move::Turn,
            Key::Down => self.direction = Move::Down,
            Key::Char('q') => panic!("C'est la panique !"),
            _ => return, // maybe do some stuff here
        }

        self.debug(format!("So the next move is : {:?}", self.direction));

        if self.direction != Move::None {
            self.compute_the_next_move();
            self.check_for_collisions();
            if self.move_is_possible {
                self.settle_the_move();
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

    fn compute_the_next_move(&mut self) {
        self.next_move_tetromino = self.tetromino;
        self.next_move_tetromino.move_it(&self.direction);
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

    fn settle_the_move(&mut self) {
        self.tetromino = self.next_move_tetromino;
        self.display_the_board();
    }

    fn freeze_and_next(&mut self) {
        for coordinate in self.tetromino.coordinates.iter() {
            self.stack[*coordinate as usize] = self.tetromino.name;
        }
        self.tetromino = Tetromino::randow_new();
        self.move_is_possible = true;
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
                self.tick_interval -= 10;
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

        write!(self.stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();

        self.stdout.write(self.debug_message.as_bytes()).unwrap();
        self.debug_message = String::new();

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
                    self.stdout.write(cell.to_printable_bytes()).unwrap();
                }
                self.stdout.write(b"|\n\r").unwrap();
            }
        }
        for _n in 0..32 {
            self.stdout.write(b"-").unwrap();
        }
        self.stdout.flush().unwrap();
    }

    fn debug<S>(&mut self, message: S)
    where
        S: ToString,
    {
        self.debug_message.push_str(&message.to_string());
        self.debug_message.push(' ');
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Move {
    Left,
    Right,
    Down,
    Turn,
    None,
}
