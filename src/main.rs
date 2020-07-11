mod game;
mod tetromino;
use game::Game;
use std::{thread, time};
use termion;

fn main() {
    let stdout = std::io::stdout();
    let mut game = Game::new(termion::async_stdin(), stdout.lock(), 800);

    println!("Use IJKL to move the pieces, Q to quit");
    thread::sleep(time::Duration::from_millis(2000));

    game.run();
}
