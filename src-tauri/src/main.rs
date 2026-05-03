// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::process::Command;
use tauri::command;

#[derive(Serialize, Deserialize, Debug)]
struct Device {
    id: u32,
    name: String,
    vendor: String,
    #[serde(rename = "type")]
    device_type: String,
    ip: String,
    mac: String,
    latency: u32,
    status: String,
    os: String,
    ports: Vec<u16>,
    uptime: String,
}

#[command]
async fn run_nmap_scan(target: String) -> Result<Vec<Device>, String> {
    // Basic ping scan to discover hosts
    // In a real app, we'd use -oX to get XML and parse it properly
    // For this initial implementation, we'll simulate the parsing of a real nmap command
    
    let output = Command::new("nmap")
        .args(["-sn", &target])
        .output()
        .map_err(|e| format!("Failed to execute nmap: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut devices = Vec::new();
    let mut current_id = 1;

    // Simple parsing logic for demonstration of 'real' data flow
    // A robust XML parser would be better for production
    for line in stdout.lines() {
        if line.contains("Nmap scan report for") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let ip = parts.last().unwrap_or(&"").trim_matches('(').trim_matches(')').to_string();
            
            devices.push(Device {
                id: current_id,
                name: if parts.len() > 5 { parts[4].to_string() } else { format!("Device {}", current_id) },
                vendor: "Unknown".to_string(),
                device_type: "unknown".to_string(),
                ip,
                mac: "00:00:00:00:00:00".to_string(),
                latency: 1,
                status: "online".to_string(),
                os: "Unknown".to_string(),
                ports: Vec::new(),
                uptime: "Unknown".to_string(),
            });
            current_id += 1;
        }
    }

    Ok(devices)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![run_nmap_scan])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
