# Setup

Follow the Bevy [setup guide](https://bevy.org/learn/quick-start/getting-started/setup)

Which is something like this:

```
sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0
sudo apt-get install mold clang
cargo run
```

# Running with trunk

You can also use trunk to run the game on web. This will automatically reload the game when you make changes.

```
cargo install trunk
trunk serve
```