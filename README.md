# NetScan Pro (mint-netscan-pro)

A high-performance, cross-platform network scanner built with Tauri and Nmap. An alternative to the Fing agent.

## Features
- **Real Data:** Fetches live network data using Nmap (no demo mode).
- **Cross-Platform:** Supports Linux (Ubuntu, Debian), Windows, macOS, and Mobile.
- **Modern UI:** Responsive and intuitive dashboard for network monitoring.
- **Vulnerability Scanning:** Detects potential security risks on your network.

## Installation Methods

### Option 1: Pre-built Installers (Easiest)
Download the installer for your platform from the [Releases](https://github.com/mintpro004/mint-netscan-pro/releases) page.

### Option 2: Installation from Source (Git Clone)
Use this method if you want to build the app manually or contribute to development.

#### 1. Prerequisites
Ensure you have the following installed:
*   **Git:** `sudo apt install git`
*   **Nmap:** `sudo apt install nmap`
*   **Node.js (v20+):** [Installation Guide](https://nodejs.org/)
*   **Rust:** `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
*   **Linux Dependencies:**
    ```bash
    sudo apt update
    sudo apt install libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf
    ```

#### 2. Clone the Repository
```bash
git clone https://github.com/mintpro004/mint-netscan-pro.git
cd mint-netscan-pro
```

#### 3. Install & Run
```bash
npm install
npm run tauri dev  # Run in development mode
# OR
npm run tauri build # Create a production installer
```
