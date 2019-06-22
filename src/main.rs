//2019-06-21
// The graal is soon to be mine.

mod tetrominoes;
mod utils;

use utils::Game;

use std::io::stdout;
use termion::async_stdin;

fn main() {
    let stdin = async_stdin();
    let stdout = stdout();
    let mut game = Game::new(stdin, stdout.lock());

    game.run();
}
