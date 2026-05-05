#!/bin/bash

# NetScan Pro - Self-Healing Setup Script
# Detects OS and installs all necessary dependencies

set -e

echo "🔍 Detecting Operating System..."
OS="$(uname -s)"
case "${OS}" in
    Linux*)     MACHINE=Linux;;
    Darwin*)    MACHINE=Mac;;
    CYGWIN*)    MACHINE=Windows;;
    MINGW*)     MACHINE=Windows;;
    *)          MACHINE="UNKNOWN:${OS}"
esac

echo "✅ OS Detected: ${MACHINE}"

if [ "$MACHINE" == "Linux" ]; then
    echo "📦 Updating system and installing dependencies..."
    
    # Detect Debian/ChromeOS specifically
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        echo "🐧 System: $PRETTY_NAME"
    fi

    sudo apt-get update
    # build-essential and libssl-dev are critical for many Rust/Node native modules
    # We include both webkit 4.0 and 4.1 to ensure compatibility across Debian versions
    sudo apt-get install -y build-essential curl wget libssl-dev git nmap \
        nodejs npm libgtk-3-dev libwebkit2gtk-4.0-dev libwebkit2gtk-4.1-dev \
        libappindicator3-dev librsvg2-dev patchelf
    
    if ! command -v rustc &> /dev/null; then
        echo "🦀 Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        # Load environment for current session
        export PATH="$HOME/.cargo/bin:$PATH"
        [ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env"
    fi

    # Ensure PATH is updated in .bashrc for future sessions if not already there
    if ! grep -q "cargo/bin" ~/.bashrc; then
        echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
    fi

    if ! command -v rustc &> /dev/null; then
        echo "❌ Rust installation failed."
        exit 1
    else
        echo "✅ Rust version: $(rustc --version)"
    fi

elif [ "$MACHINE" == "Mac" ]; then
    echo "📦 Installing dependencies via Homebrew..."
    if ! command -v brew &> /dev/null; then
        echo "🍺 Installing Homebrew first..."
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    fi
    brew install git nmap node rust

elif [ "$MACHINE" == "Windows" ]; then
    echo "📦 Please use PowerShell for Windows installation."
    exit 1
fi

echo "🚀 Setting up NetScan Pro..."
npm install

echo "✨ All set! Run 'npm run tauri dev' to start."
