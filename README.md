# NetScan Pro

A high-performance, cross-platform network scanner built with Tauri and Nmap. An alternative to the Fing agent.

## Features
- **Real Data:** Fetches live network data using Nmap (no demo mode).
- **Cross-Platform:** Supports Linux (Ubuntu, Debian), Windows, macOS, and Mobile.
- **Modern UI:** Responsive and intuitive dashboard for network monitoring.
- **Vulnerability Scanning:** Detects potential security risks on your network.

## Installation

### Linux
1. Install Nmap: `sudo apt install nmap`
2. Download the `.deb` or `AppImage` from the [Releases](https://github.com/YOUR_USERNAME/YOUR_REPO/releases) page.
3. Install the `.deb`: `sudo dpkg -i netscan-pro.deb`

### Windows
1. Install [Nmap for Windows](https://nmap.org/download.html#windows).
2. Download the `.msi` or `.exe` from the Releases page.
3. Run the installer.

### macOS
1. Install Nmap using Homebrew: `brew install nmap`.
2. Download the `.dmg` from the Releases page.
3. Drag to Applications.

## Development
1. Install Rust and Node.js.
2. Clone the repo.
3. Run `npm install`.
4. Run `npm run tauri dev`.
