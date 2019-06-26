//2019-06-21
// The graal is soon to be mine.

mod utils;

use utils::Game;

use std::io::stdout;
use termion::async_stdin;
use std::{thread, time};

fn main() {
    let stdin = async_stdin();
    let stdout = stdout();
    let mut game = Game::new(stdin, stdout.lock());

    println!("Use IJKL like so:\n
          I: Turn
J: left   K: down   L: right\n

Q to panic");
    thread::sleep(time::Duration::from_millis(2000));

    game.run();
}
