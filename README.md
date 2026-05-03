# NetScan Pro (mint-netscan-pro)

A high-performance, cross-platform network scanner built with Tauri and Nmap. An alternative to the Fing agent.

## Features
- **Real Data:** Fetches live network data using Nmap (no demo mode).
- **Cross-Platform:** Supports Linux (Ubuntu, Debian), Windows, macOS, and Mobile.
- **Modern UI:** Responsive and intuitive dashboard for network monitoring.
- **Vulnerability Scanning:** Detects potential security risks on your network.

## Installation

### 🐧 Linux (Debian, Ubuntu, etc.)
1.  **Install Nmap:**
    ```bash
    sudo apt update && sudo apt install nmap -y
    ```
2.  **Download the App:** Go to the [Releases](https://github.com/mintpro004/mint-netscan-pro/releases) page and download the `.deb` file.
3.  **Install:**
    ```bash
    sudo dpkg -i netscan-pro_*.deb
    sudo apt install -f  # Fix any missing dependencies
    ```

### 💻 ChromeOS (Chromebooks)
*Note: Requires the Linux development environment (Crostini) to be enabled in Settings.*
1.  **Open Terminal** in your Chromebook.
2.  **Install Nmap:**
    ```bash
    sudo apt update && sudo apt install nmap -y
    ```
3.  **Download the App:** Download the Linux `.deb` file from the Releases page and move it to the "Linux files" folder in your Files app.
4.  **Install:**
    ```bash
    sudo dpkg -i netscan-pro_*.deb
    sudo apt install -f
    ```
5.  **Launch:** The app will appear in your "Linux apps" folder in the ChromeOS Launcher.

### 🪟 Windows
1.  **Install Nmap:** Download and run the [Nmap Windows Installer](https://nmap.org/download.html#windows).
2.  **Download the App:** Download the `.msi` or `.exe` installer from the Releases page.
3.  **Run Installer:** Double-click the downloaded file and follow the prompts.

### 🍎 macOS
1.  **Install Nmap:** (Requires [Homebrew](https://brew.sh/))
    ```bash
    brew install nmap
    ```
2.  **Download the App:** Download the `.dmg` from the Releases page.
3.  **Install:** Open the `.dmg` and drag "NetScan Pro" to your Applications folder.

### 📱 Mobile (Android/iOS)
Mobile builds are currently available via the source code. To generate a mobile package:
1.  Follow the [Tauri Mobile Guide](https://v2.tauri.app/start/mobile/).
2.  Run `npm run tauri android init` or `npm run tauri ios init`.
3.  Build the package using `npm run tauri android build`.

## Development
1. Install Rust and Node.js.
2. Clone the repo.
3. Run `npm install`.
4. Run `npm run tauri dev`.
