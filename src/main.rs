//2019-06-21
// The graal is soon to be mine.

mod utils;

use utils::Game;

use std::io::stdout;

use std::{thread, time};
use termion::async_stdin;
fn main() {
    let stdin = async_stdin();
    let stdout = stdout();
    let mut game = Game::new(stdin, stdout.lock());

    println!(
        "Use IJKL like so:\n
          I: Turn
J: left   K: down   L: right\n

Q to panic"
    );
    thread::sleep(time::Duration::from_millis(2000));

    game.run();
}
