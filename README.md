# A minimal rust implementation of Tetris

The goal of the exercise is to run the most minimalist terminal-running Tetris. 
Rendering is done using Termion and that's about it.

```
|                              |
|                              |
|                              |
|                              |
|                              |
|                              |
|                              |
|            OOOOOO            |
|            OOOOOO            |
|            OOOOOO            |
|            OOOOOO            |
|                              |
|                              |
|                              |
|                              |
|                              |
|                              |
|                              |
|                              |
|                              |
|                              |
|                           III|
|                           III|
|                           III|
|                           III|
|                           III|
|                           III|
|   ZZZ               ZZZ   III|
|   ZZZ               ZZZ   III|
|ZZZZZZTTT         ZZZZZZ   LLL|
|ZZZZZZTTT         ZZZZZZ   LLL|
|ZZZTTTTTTTTT      ZZZLLLLLLLLL|
|ZZZTTTTTTTTT      ZZZLLLLLLLLL|
--------------------------------
```

## Install and run

Be sure so [install Rust](https://www.rust-lang.org/learn/get-started) first.

Clone the repository, go the directory, run with cargo

```sh
git clone https://github.com/Keksoj/tetris.git
cd tetris
cargo run
```

## The challenges and the solutions

I first thought of using (x, y) coordinates for representing the board and the shapes, but it is much easier to have just one axis and chuck it like so :

```rust
 200 201 202 203 204 205 206 207 208 209
 190 191 192 193 194 195 196 197 198 199
 // ------
  20  21  22  23  24  25  26  27  28  29
  10  11  12  13  14  15  16  17  18  19
   0   1   2   3   4   5   6   7   8   9
```

And a Tetromino is just a set of 4 coordinates:

```rust
struct Tetromino {
  blocks: [u8; 4],
}
```

**Moving the pieces**

* move a Tetromino one row down by doing `-= 10` on all its coordinate. 
* `-= 1` to push it to the left, `+= 1` to the right
* Turning is done by changing the coordinates depending on the shape, by and recording the "spin", the number of turns.

**Detecting collisions**

There are a number of elegant ways to do this but I chose the rookie method : 

1. compute the new coordinates
2. check for overlapping with the walls, the bottom, other pieces
3. performing the move or not

**Ticking down AND accepting moves from the player**

Wether or not it's moved around by the player, the Tetromino has to tick down every, say, one second.
This is a case study for multi-threading and other nightmares. 
The easy way is to do this is to 

1. set a mutable time stamp at every tick
2. check the elapsed time since the last tick, and if it's time :
3. perform the tick
4. reset the time stamp to now


```rust
// set the mutable timestamp to now
let mut last_tick = std::time::SystemTime::now();

  loop {
      // takes user input and move the shape accordingly
      self.take_directions();

      // check if we reached 1000 milliseconds
      if last_tick.elapsed().unwrap().as_millis() >= 1000 {
          
          // perform the tick
          self.tick();
          
          // reset the time stamp
          last_tick = std::time::SystemTime::now();
      }
  }
```
