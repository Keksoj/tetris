//2019-06-21
// The graal is soon to be mine.

mod utils;
use std::{thread, time};
use termion;
use crate::utils::Game;

fn main() {
    let stdout = std::io::stdout();
    let mut game = Game::new(termion::async_stdin(), stdout.lock(), 800);
    // let mut game = Game::new(termion::async_stdin(), stdout.lock());

    println!("Use IJKL to move the pieces, Q to quit");
    thread::sleep(time::Duration::from_millis(2000));

    game.run();
}
