#!/bin/bash
# Script to build WASM, update web files, and restart the server

set -e

echo "Building WASM..."
cd /home/sparks/Themathar/themathar_game
cargo build --release --target wasm32-unknown-unknown

echo "Generating wasm-bindgen bindings..."
wasm-bindgen target/wasm32-unknown-unknown/release/themathar_game.wasm --out-dir web --target web

echo "Killing existing server..."
pkill -f "python3 -m http.server 8000" || true
sleep 1

echo "Starting server..."
cd /home/sparks/Themathar/themathar_game/web
python3 -m http.server 8000 > /dev/null 2>&1 &
echo "Server started on http://localhost:8000"

