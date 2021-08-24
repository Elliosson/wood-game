A client for my wood-game

Run it on desktop with: cargo run --release

To build a wasm client use: wasm-pack build --target web --release
Then serve the wasm client with a http server, for example run: http-server .

This client is build with the bevy engine.
The address of the server is hardcoded.