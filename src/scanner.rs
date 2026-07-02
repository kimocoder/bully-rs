use crate::socket::RawSocket;
use log::info;
use std::process::Command;

fn check_monitor_mode(interface: &str) -> anyhow::Result<()> {
    let output = Command::new("iw")
        .args(["dev", interface, "info"])
        .output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.contains("type monitor") {
        Ok(())
    } else {
        anyhow::bail!(
            "Interface {} is not in monitor mode.\n\
            Please set it to monitor mode using:\n\
            sudo ip link set {} down && sudo iw dev {} set type monitor && sudo ip link set {} up",
            interface, interface, interface, interface
        );
    }
}

pub fn start_scanner(socket: &RawSocket, interface: &str) -> anyhow::Result<()> {
    check_monitor_mode(interface)?;
    info!("Scanner active. Searching for WPS-enabled APs...");
    let mut buf = [0u8; 2048];
    loop {
        match socket.recv_frame(&mut buf) {
            Ok(len) => {
                // Simple heuristic: check for WPS IE OUI: 00 50 f2 04
                if buf[..len].windows(4).any(|window| window == [0x00, 0x50, 0xf2, 0x04]) {
                    info!("Found potential WPS-enabled AP!");
                }
            }
            Err(e) => {
                log::error!("Error receiving frame: {}", e);
            }
        }
    }
}
