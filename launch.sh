#!/bin/bash

# NetScan Pro - Easy Launch Script
# This script ensures everything is configured before launching the app.

echo "🚀 Starting NetScan Pro..."

# 1. Check if Node modules exist
if [ ! -d "node_modules" ]; then
    echo "📦 Missing node_modules, running npm install..."
    npm install
fi

# 2. Check if Rust is in PATH
if ! command -v rustc &> /dev/null; then
    if [ -f "$HOME/.cargo/env" ]; then
        source "$HOME/.cargo/env"
    else
        echo "⚠️ Rust not found. Running setup..."
        ./setup.sh
        source "$HOME/.cargo/env"
    fi
fi

# 3. Launch the app in dev mode
echo "✨ Launching application..."
npm run tauri dev
