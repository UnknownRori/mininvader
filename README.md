# Mininvader

A space invader inspired game but instead of traditional space invader it come with bullet hell (STG) or danmaku genre.
STG are fast-paced game focused on pattern recognition and mastery through practice.

Mininvader is portable and is written in Rust 1.71 using miniquad renderer. 
It's officially support Windows, Linux, macOS, and WebGL-enabled browsers such as Firefox and Chromium-based browser.

## Installation

> [!WARNING]
> Stable pre-compiled is not yet available on usual download/release tab, you can install it via `cargo install` or `cargo build`
> The pre-compiled latest binary is available on github action tab.

## Development

Make sure you have Rust 1.71 installed.

```sh
# Clone and enter project directory
git clone https://github.com/UnknownRori/mininvader
cd mininvader

# Build the project for desktop
cargo build
# Build the project for web
cargo build --target wasm32-unknown-unknown

# Run the game
cargo run
```

