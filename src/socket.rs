use std::io;
use std::os::unix::io::RawFd;
use libc::{socket, AF_PACKET, SOCK_RAW, ETH_P_ALL, if_nametoindex, bind};
use anyhow::{Result, anyhow};
use std::mem;

#[repr(C)]
pub struct SockAddrLl {
    pub sll_family: libc::sa_family_t,
    pub sll_protocol: i16,
    pub sll_ifindex: i32,
    pub sll_hatype: u16,
    pub sll_pkttype: u8,
    pub sll_halen: u8,
    pub sll_addr: [u8; 8],
}

pub struct RawSocket {
    fd: RawFd,
}

impl RawSocket {
    pub fn new(interface: &str) -> Result<Self> {
        let fd = unsafe { socket(AF_PACKET, SOCK_RAW, (ETH_P_ALL as u16).to_be() as i32) };
        if fd < 0 {
            return Err(anyhow!("Failed to create raw socket: {}", io::Error::last_os_error()));
        }

        let if_name = std::ffi::CString::new(interface).unwrap(); let ifindex = unsafe { if_nametoindex(if_name.as_ptr()) };
        if ifindex == 0 {
            unsafe { libc::close(fd) };
            return Err(anyhow!("Invalid interface name: {}", interface));
        }

        let mut sll: SockAddrLl = unsafe { mem::zeroed() };
        sll.sll_family = AF_PACKET as libc::sa_family_t;
        sll.sll_protocol = (ETH_P_ALL as u16).to_be() as i16;
        sll.sll_ifindex = ifindex as i32;

        let bind_res = unsafe {
            bind(
                fd,
                &sll as *const SockAddrLl as *const libc::sockaddr,
                mem::size_of::<SockAddrLl>() as libc::socklen_t,
            )
        };

        if bind_res < 0 {
            unsafe { libc::close(fd) };
            return Err(anyhow!("Failed to bind socket to interface {}: {}", interface, io::Error::last_os_error()));
        }
        
        Ok(RawSocket { fd })
    }

    pub fn send_frame(&self, frame: &[u8]) -> Result<usize> {
        let bytes_sent = unsafe {
            libc::send(self.fd, frame.as_ptr() as *const libc::c_void, frame.len(), 0)
        };
        if bytes_sent < 0 {
            return Err(anyhow!("Failed to send frame: {}", io::Error::last_os_error()));
        }
        Ok(bytes_sent as usize)
    }

    pub fn recv_frame(&self, buf: &mut [u8]) -> Result<usize> {
        let bytes_recv = unsafe {
            libc::recv(self.fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len(), 0)
        };
        if bytes_recv < 0 {
            return Err(anyhow!("Failed to receive frame: {}", io::Error::last_os_error()));
        }
        Ok(bytes_recv as usize)
    }
}

impl Drop for RawSocket {
    fn drop(&mut self) {
        unsafe { libc::close(self.fd) };
    }
}
