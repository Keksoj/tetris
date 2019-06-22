use crate::tetrominoes::{Name, Tetromino};
use std::io::{Read, Write};
use std::{thread, time};
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::{clear, cursor};

#[derive(Debug)]
pub struct Game<R, W: Write> {
    pub stdout: W,
    pub stdin: R,
    pub speed: u64,
    pub tetromino: Tetromino,
    // the "stack" is where the non-moving tetrominoes are stacked
    pub stack: [[u8; 10]; 21],
    // the board is where the stack AND the moving tetrominoes are displayed
    pub board: [[u8; 10]; 21],
}

impl<R: Read, W: Write> Game<R, W> {
    // Set a new empty game
    pub fn new(stdin: R, stdout: W) -> Game<R, RawTerminal<W>> {
        let new_game = Game {
            stdout: stdout.into_raw_mode().unwrap(),
            stdin: stdin,
            speed: 700,
            tetromino: Tetromino::create(Name::T),
            stack: [[0; 10]; 21],
            board: [[0; 10]; 21], // fill the board with zeroes
        };
        new_game
    }

    pub fn run(&mut self) {
        loop {
            self.take_directions();
            // this will call push_left(), push_right(), turn() or tick_down()
            self.tick_down();
            // This will call freeze_and_next() if
            self.display_the_board();
            self.check_if_game_over();
            self.display_the_board();
            thread::sleep(time::Duration::from_millis(self.speed));
        }
    }

    fn take_directions(&mut self) {
        let mut b = [0];
        self.stdin.read(&mut b).unwrap();
        match b[0] {
            b'i' => self.turn(),
            b'j' => self.push_left(),
            b'l' => self.push_right(),
            b'k' => self.tick_down(),
            b'q' => panic!("c'est la panique !"),
            _ => {}
        }
        // I stil have to understand what this does.
        self.stdout.flush().unwrap();
    }

    fn it_collides(&mut self, next_move: NextMove) -> bool {
        let blocks = self.tetromino.blocks;
        match next_move {
            NextMove::GoLeft => {
                // check for collisions with the left wall
                for i in 0..4 {
                    if self.tetromino.blocks[i][1] == 0 {
                        return true;
                    }
                }
                // check for collisions with the stack
                // this checks that all cells at the left of the tetromino are empty
                for i in blocks.iter() {
                    if self.stack[i[0] as usize][(i[1] - 1) as usize] != 0 {
                        return true;
                    }
                }
            }
            NextMove::GoRight => {
                // check for collisions with the right wall
                for i in 0..4 {
                    if self.tetromino.blocks[i][1] == 9 {
                        return true;
                    }
                }
                // check for collisions with the stack
                // this checks that all cells at the left of the tetromino are empty
                for i in self.tetromino.blocks.iter() {
                    if self.stack[i[0] as usize][(i[1] + 1) as usize] != 0 {
                        return true;
                    }
                }
            }
            NextMove::GoDown => {
                // check for collision with the bottom of the frame
                for i in 0..4 {
                    if self.tetromino.blocks[i][0] == 0 {
                        return true;
                    }
                }
                // check for collisions with the stack
                // this checks that all cells at the left of the tetromino are empty
                for i in self.tetromino.blocks.iter() {
                    if self.stack[(i[0] - 1) as usize][i[1] as usize] != 0 {
                        return true;
                    }
                }
            }
            NextMove::Turn => {
                // get the coordinates of the tetromino IF turned
                let simulation = self.tetromino.simulate_a_turn();

                // Here, have some boilerplate code
                // check for collisions with the walls
                for i in 0..4 {
                    if simulation[i][1] == 0    // left wall
                        || simulation[i][1] == 9 // right wall
                        || simulation[i][0] == 0 {  // bottom
                            return true;
                        }
                }
                // Check for collision with the stack
                for i in simulation.iter() {
                    if self.stack[i[0] as usize][i[1] as usize] != 0 {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn turn(&mut self) {
        // check for collisions with the frame
        if self.it_collides(NextMove::Turn) {
            return;
        } else {
            self.tetromino.turn();
            self.display_the_board()
        }
    }

    fn push_left(&mut self) {
        if self.it_collides(NextMove::GoLeft) {
            return;
        } else {
            self.tetromino.push_left();
            self.display_the_board()
        }
    }

    fn push_right(&mut self) {
        if self.it_collides(NextMove::GoRight) {
            return;
        } else {
            self.tetromino.push_right();
            self.display_the_board()
        }
    }

    fn tick_down(&mut self) {
        if self.it_collides(NextMove::GoDown) {
            self.freeze_and_next()
        } else {
            self.tetromino.pull_down();
            self.display_the_board();
        }
    }

    fn freeze_and_next(&mut self) {
        // this writes the tetromino footprint on the "stack"
        for i in self.tetromino.blocks.iter() {
            self.stack[i[0] as usize][i[1] as usize] = self.tetromino.footprint;
        }
        // This calls the new tetromino
        self.tetromino = Tetromino::randow_new_tetromino();
    }

    pub fn display_the_board(&mut self) {
        // Draw the stack (this erases the previously appearing tetromino)
        for i in 0..20 {
            self.board[i] = self.stack[i]
        }

        // Draw the tetromino
        for i in self.tetromino.blocks.iter() {
            self.board[i[0] as usize][i[1] as usize] = self.tetromino.footprint;
        }

        write!(self.stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();

        let mut counter: usize = 19;
        loop {
            self.stdout.write(b"|").unwrap();

            for &cell in &self.board[counter] {
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
            self.stdout.write(b"|").unwrap();
            self.stdout.write(b"\n\r").unwrap();
            if counter == 0 {
                break;
            } else {
                counter -= 1
            }
        }
        for _n in 0..12 {
            self.stdout.write(b"-").unwrap();
        }
        self.stdout.flush().unwrap();
        // write!(self.stdout, "{}", cursor::Hide).unwrap();
    }

    fn check_if_game_over(&mut self) {
        if self.stack[16] != [0; 10] {
            panic!("Game over!")
        }
    }
}

enum NextMove {
    GoLeft,
    GoRight,
    GoDown,
    Turn,
}
