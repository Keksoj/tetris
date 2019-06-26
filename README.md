# A minimal rust implementation of Tetris

This exercise fullfills a threefold learning puropose : refactoring, refactoring, and refactoring.

## What data type to chose ?

I first thought every cell would be best represented by a `u8`, either 0 (empty) or 1 (filled by a T shape) or 2 (filled by a I shape) etc.
The board was an array of byte arrays:

```rust
board: [[u8; 10]; 21]
```

Whichs makes for nice (x, Y) coordinates:

```rust
board[x_coordinate][y_coordinate] = assign_a_value;
```

Thus the tetrominoes (or shapes) would be a struct of four points:

```rust
struct Tetromino {
  one: (u8, u8),
  two: (u8, u8),
  three: (u8, u8),
  four: (u8, u8),
}
```

This is refactorable into

```rust
struct Tetromino {
  blocks: [[u8; 2]; 4],
}
```

However, having a set of coordinates is more painfull than helpfull. Let's follow the lead of the UNIX creators, always striving for removing code, rather than adding any. So I removed some brackets:

```rust
board: [u8; 210]
```

Which, once chunked, represents the board like this:

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

The gods of mathematics are with us, we can move a Tetromino one row down by doing `-= 10` on all its coordinate. `+= 1` to push it to the right, and so on.

The whole code has been refactored on and on, and will be, again and again. Getting the game working is done, the goal now is to learn the community standards of a code that is clean and easily understandable.
