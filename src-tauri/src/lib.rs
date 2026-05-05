use serde::{Deserialize, Serialize};
use quick_xml::events::Event;
use quick_xml::reader::Reader;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Device {
    pub id: u32,
    pub name: String,
    pub vendor: String,
    #[serde(rename = "type")]
    pub device_type: String,
    pub ip: String,
    pub mac: String,
    pub latency: u32,
    pub status: String,
    pub os: String,
    pub ports: Vec<u16>,
    pub uptime: String,
}

pub fn parse_nmap_xml(xml: &str) -> Result<Vec<Device>, String> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut devices = Vec::new();
    let mut current_device = Device::default();
    let mut current_id = 1;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
                    b"host" => {
                        current_device = Device::default();
                        current_device.id = current_id;
                        current_device.status = "online".to_string();
                    }
                    b"status" => {
                        for attr in e.attributes() {
                            let attr = attr.map_err(|e| e.to_string())?;
                            if attr.key.as_ref() == b"state" {
                                current_device.status = String::from_utf8_lossy(&attr.value).to_string();
                            }
                        }
                    }
                    b"address" => {
                        let mut addr = String::new();
                        let mut addr_type = String::new();
                        let mut vendor = String::new();

                        for attr in e.attributes() {
                            let attr = attr.map_err(|e| e.to_string())?;
                            match attr.key.as_ref() {
                                b"addr" => addr = String::from_utf8_lossy(&attr.value).to_string(),
                                b"addrtype" => addr_type = String::from_utf8_lossy(&attr.value).to_string(),
                                b"vendor" => vendor = String::from_utf8_lossy(&attr.value).to_string(),
                                _ => {}
                            }
                        }

                        if addr_type == "ipv4" {
                            current_device.ip = addr;
                        } else if addr_type == "mac" {
                            current_device.mac = addr;
                            current_device.vendor = if vendor.is_empty() { "Unknown".to_string() } else { vendor };
                        }
                    }
                    b"hostname" => {
                        for attr in e.attributes() {
                            let attr = attr.map_err(|e| e.to_string())?;
                            if attr.key.as_ref() == b"name" {
                                current_device.name = String::from_utf8_lossy(&attr.value).to_string();
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                if e.name().as_ref() == b"host" {
                    if current_device.name.is_empty() {
                        current_device.name = format!("Device {}", current_id);
                    }
                    devices.push(current_device.clone());
                    current_id += 1;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("XML Error: {}", e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(devices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_nmap_xml() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE nmaprun>
<nmaprun scanner="nmap" args="nmap -sn -oX - 192.168.1.0/24" start="1714841700" version="7.93" xmloutputversion="1.05">
<host><status state="up" reason="arp-response" 선보="0"/>
<address addr="192.168.1.1" addrtype="ipv4"/>
<address addr="A4:C3:F0:11:22:33" addrtype="mac" vendor="ASUS"/>
<hostnames><hostname name="router.home" type="PTR"/></hostnames>
<times srtt="1000" rttvar="5000" to="100000"/>
</host>
<host><status state="up" reason="localhost-response" 선보="0"/>
<address addr="192.168.1.10" addrtype="ipv4"/>
<hostnames><hostname name="MacBook-Pro" type="user"/></hostnames>
</host>
<runstats><finished time="1714841705" timestr="Mon May  4 21:35:05 2026" summary="Nmap done: 256 IP addresses (2 hosts up) scanned in 5.12 seconds" elapsed="5.12" exit="success"/><hosts up="2" down="254" total="256"/>
</runstats>
</nmaprun>"#;

        let devices = parse_nmap_xml(xml).unwrap();
        assert_eq!(devices.len(), 2);
        
        assert_eq!(devices[0].ip, "192.168.1.1");
        assert_eq!(devices[0].mac, "A4:C3:F0:11:22:33");
        assert_eq!(devices[0].vendor, "ASUS");
        assert_eq!(devices[0].name, "router.home");
        
        assert_eq!(devices[1].ip, "192.168.1.10");
        assert_eq!(devices[1].name, "MacBook-Pro");
        assert_eq!(devices[1].mac, "");
    }
}
