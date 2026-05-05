// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::process::Command;
use tauri::command;
use netscan_pro_lib::{Device, parse_nmap_xml};

#[command]
async fn run_nmap_scan(target: String) -> Result<Vec<Device>, String> {
    let output = Command::new("nmap")
        .args(["-sn", "-oX", "-", &target])
        .output()
        .map_err(|e| format!("Failed to execute nmap: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let xml_data = String::from_utf8_lossy(&output.stdout);
    parse_nmap_xml(&xml_data)
}

#[derive(Serialize, Deserialize, Debug)]
struct SystemStatus {
    nmap_installed: bool,
    nmap_version: String,
    rust_installed: bool,
    rust_version: String,
    is_admin: bool,
    os: String,
}

#[command]
async fn check_system() -> Result<SystemStatus, String> {
    let os = std::env::consts::OS.to_string();
    
    // Check if nmap is installed
    let nmap_check = Command::new("nmap")
        .arg("--version")
        .output();

    let (nmap_installed, nmap_version) = match nmap_check {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout)
                .lines()
                .next()
                .unwrap_or("Unknown")
                .to_string();
            (true, version)
        },
        Err(_) => (false, "Not Found".to_string()),
    };

    // Check if rust is installed
    let rust_check = Command::new("rustc")
        .arg("--version")
        .output();

    let (rust_installed, rust_version) = match rust_check {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_string();
            (true, version)
        },
        Err(_) => (false, "Not Found".to_string()),
    };

    // Check for admin/root permissions
    #[cfg(not(windows))]
    let is_admin = unsafe { libc::getuid() == 0 };
    #[cfg(windows)]
    let is_admin = true; // Placeholder for Windows admin check

    Ok(SystemStatus {
        nmap_installed,
        nmap_version,
        rust_installed,
        rust_version,
        is_admin,
        os,
    })
}

#[command]
async fn fix_dependency(handle: tauri::AppHandle, dependency: String) -> Result<String, String> {
    let os = std::env::consts::OS;
    
    match dependency.as_str() {
        "nmap" => {
            if os == "linux" {
                let status = Command::new("pkexec")
                    .args(["apt-get", "install", "-y", "nmap"])
                    .status()
                    .map_err(|e| format!("Failed to launch pkexec: {}", e))?;
                
                if status.success() {
                    Ok("Nmap installed successfully".to_string())
                } else {
                    Err("Installation failed or cancelled".to_string())
                }
            } else if os == "windows" {
                let status = Command::new("winget")
                    .args(["install", "Insecure.Nmap"])
                    .status()
                    .map_err(|e| format!("Failed to launch winget: {}", e))?;
                
                if status.success() {
                    Ok("Nmap installed via winget".to_string())
                } else {
                    Err("Winget installation failed".to_string())
                }
            } else {
                Err(format!("Auto-fix not supported for OS: {}", os))
            }
        },
        "rust" => {
            if os == "linux" || os == "macos" {
                let status = Command::new("sh")
                    .arg("-c")
                    .arg("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y")
                    .status()
                    .map_err(|e| format!("Failed to run rustup installer: {}", e))?;
                
                if status.success() {
                    Ok("Rust installed successfully. Please restart the app to update PATH.".to_string())
                } else {
                    Err("Rust installation failed".to_string())
                }
            } else {
                Err(format!("Auto-fix not supported for OS: {}", os))
            }
        },
        _ => Err("Unknown dependency".to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![run_nmap_scan, check_system, fix_dependency])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
