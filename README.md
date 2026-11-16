# Drift

A small, quiet 2D obstacle garden built with Bevy. Guide a mint square around
copper obstacles and collect gold squares to increase the score shown at the
bottom left. Each new round restores the items and resets the score; the score
is not saved. There is no time limit.

## Install and run

Install Rust 1.97.1 or newer with Cargo. On Ubuntu, install the X11 and Vulkan
development/runtime packages:

```sh
sudo apt install build-essential pkg-config libx11-dev libxkbcommon-dev libvulkan1 mesa-vulkan-drivers
cargo build --locked -j 2
cargo run --locked
```

The application uses Bevy 0.20.1 with only the features needed for sprites, text,
UI, keyboard input, and an X11 window. No assets or services need configuring.
The project began with `cargo init --bin --name drift --edition 2024 .`.
Debug information is disabled to keep local builds smaller; dependency
optimization is left at Cargo's default to keep compilation time down.

## Controls

- Menu: Enter opens the garden; S opens settings.
- Garden: WASD or arrow keys move. Obstacles and the garden edge block movement.
  Collect gold squares to add one point each to the score HUD. Entering the garden
  starts a new round with all items and zero score; score is not saved.
- Settings: F toggles the FPS display and saves immediately.
- Esc returns to the menu from either screen. Close the window to quit.

## Device settings

The FPS preference is the only saved feature. Settings are JSON with a top-level
`version` field (currently `1`), stored using `directories::ProjectDirs` for
organization `Klyve` and application `Drift`:

- Linux: `$XDG_DATA_HOME/drift/settings.json`, normally `~/.local/share/drift/settings.json`.
- macOS: `~/Library/Application Support/Klyve.Drift/settings.json`.
- Windows: `%APPDATA%\Klyve\Drift\data\settings.json`.

The build is configured for Linux X11. No environment variables are required.
Missing settings use defaults. Malformed files, unsupported versions, or read
errors also fall back to defaults with a startup message. Save failures appear
in the settings screen; the preference still works for the current session.
Writes replace the JSON file through a temporary file in the same directory.
Nothing is sent over the network.

## Development

```sh
cargo test --locked -j 2
cargo clippy --locked --all-targets -j 2 -- -D warnings
cargo fmt --check
cargo build --locked -j 2
cargo run --locked
```

Tests cover settings round trips, missing and invalid files, state transitions,
time-based movement, diagonal speed, obstacle collisions, and garden boundaries.
They also cover item collection, scoring once per item, round score resets,
scoring only while playing, and reachable item positions. The score HUD shows
the current round only; scores are not saved.
The Bevy system tests use `MinimalPlugins` without a window or renderer.

For a short display-server smoke run on Linux:

```sh
xvfb-run -a timeout 15s cargo run --locked
```

A timeout exit after the window opens is normal. Use `cargo clean` to remove
compiled files when finished.

## License

MIT. Copyright (c) 2026 Klyve.
