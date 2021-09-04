# Ironrift
A sci-fi first/third-person battlefield-style shooter.
This was made as an experiment in Rust and Bevy.

## Building
Requirements: `rust` and nightly `cargo` via `rustup` and optionally `git`.  

```
git clone https://github.com/GreenXenith/ironrift
cd ironrift
cargo +nightly run --release
```
Run using the generated executable in `ironrift/target/release` (must be run at or below asset directory level).  

## Controls
* Mouse to aim
* `WASD` to move
* `LMB` to shoot
* `ESC` to exit

## Known Bugs
* May fail to grab cursor (Linux)
* NPCs may evaporate (and possibly crash)
* Physics engine may crash due to something being out of bounds
* May be able to jump infinitely
  
I have no idea why these occur. If they do, just run the game again.
