# Snake game made with Bevy in Rust

You can play at https://agustinramirodiaz.github.io/snake_bevy/

About the game:

- Similar to snake
- Eat the apple to grow
- Hitting yourself or others will shrink you
- 1-4 players

# Running the game

Requires Rust and Cargo to be installed.

```
cargo run
```

## Running with trunk

You can also use trunk to run the game on web. This will automatically reload the game when you make changes.

```
cargo install trunk
trunk serve
```

# Contributing

Issues, feature requests and pull requests are welcome!

# License

Do whatever you want with this code

# Things I want to do next

- [ ] Add pause/play
- [ ] Configurable keybindings
- [ ] Add sprites for

  - [ ] snake head
  - [ ] snake body
  - [x] apple

- [x] Add a win condition
- [x] Add some basic AI

# Ideas

- power ups
  - speed up
  - slow down others
  - invincible for a short time
  - shield
- different maps
  - wrap around
  - walls
  - obstacles
- different game modes
  - by win condition
    - hold first position for an amount of time
    - reach X length first
    - reach X difference in length to the second player
  - by game rules
    - no walls
    - no obstacles
    - power up configuration
