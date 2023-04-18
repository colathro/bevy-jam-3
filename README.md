# The violence portrayed in this game does not represent my personal beliefs. There is zero intention to have any polictical swing or naratives to be derived from the story or gameplay of this game. It's just a funny name, and outrageous concept. The main goal of this game is to share a pattern of project layout, state management, customization of the rapier 2d plugin systems, and the patterns for developing a game in an ECS framework.

# side-effects
Entry for the Bevy Jam 3 - https://itch.io/jam/bevy-jam-3

# Ligma: The fourth trimester

# running

```cargo run --target wasm32-unknown-unknown```

# generating files for hosting

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/
