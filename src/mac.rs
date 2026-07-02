use std::fmt;
use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacAddress(pub [u8; 6]);

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
               self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5])
    }
}

impl MacAddress {
    pub fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 6 {
            return Err(anyhow!("Invalid MAC address format. Expected XX:XX:XX:XX:XX:XX"));
        }
        let mut addr = [0u8; 6];
        for (i, part) in parts.iter().enumerate() {
            addr[i] = u8::from_str_radix(part, 16).map_err(|_| anyhow!("Invalid hex digit in {}", part))?;
        }
        Ok(MacAddress(addr))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 6 {
            return Err(anyhow!("Insufficient bytes for MAC address"));
        }
        let mut addr = [0u8; 6];
        addr.copy_from_slice(&bytes[0..6]);
        Ok(MacAddress(addr))
    }
}
