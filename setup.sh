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
    sudo apt-get update
    sudo apt-get install -y git nmap nodejs npm libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf
    
    if ! command -v rustc &> /dev/null; then
        echo "🦀 Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source $HOME/.cargo/env
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
