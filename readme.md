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

# TODO:

- [ ] fix deployment https://trunkrs.dev/advanced/

# Things I want to do next

- [ ] Add world sync for systems to decouple data from rendering, like having a first set of systems do calculations and a second set to render based on the updated data
- [ ] Add pause/play
- [ ] Hanle gamepads
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

# Notes

I've been able to build for web with [this guide](https://dev.to/sbelzile/making-games-in-rust-deploying-a-bevy-app-to-the-web-1ahn) too, but I didn't publish it like this since there wasn't any difference with the trunk version.
