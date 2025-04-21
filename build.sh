#!/bin/bash

# Build the native library
echo "Building native library..."
cargo build --features native

# Build the WASM library
echo "Building WASM library..."
cargo build --target wasm32-unknown-unknown --features wasm-bindgen

# Build the examples
echo "Building examples..."
cargo build --examples

# Run the basic example
echo "Running basic example..."
PINATA_API_KEY=your_api_key PINATA_SECRET_KEY=your_secret_key cargo run --example basic

# Build the WASM example
echo "Building WASM example..."
cargo build --example wasm --target wasm32-unknown-unknown --features wasm-bindgen

# Generate WASM bindings
echo "Generating WASM bindings..."
wasm-bindgen --target web --out-dir examples --out-name wasm target/wasm32-unknown-unknown/debug/examples/wasm.wasm

echo "Build complete!"
echo "To test the WASM example, serve the examples directory using a web server and open index.html in a browser." 