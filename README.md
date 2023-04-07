# side-effects
Entry for the Bevy Jam 3 - https://itch.io/jam/bevy-jam-3

# Ligma: The fourth trimester

You are a woman who has been given a breast enlargement medication by a shady doctor. Though you are unaware that the medication has a sinister side-effect. Zombies begin to appear and you need to destroy them.

# running

```cargo run --target wasm32-unknown-unknown```

# generating files for hosting

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/
