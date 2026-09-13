use std::net::TcpListener;

pub const SPICE_PORT_START: u16 = 5900;
pub const RDP_PORT_START: u16 = 3389;

pub fn next_free_port(start: u16) -> u16 {
    let mut port = start;
    loop {
        // Probe whether we can bind to 127.0.0.1:port
        // Note: Eliminates subprocess overhead of `ss`, but the TOCTOU window remains
        // until the target service (QEMU) binds to it.
        if TcpListener::bind(("127.0.0.1", port)).is_ok() {
            return port;
        }
        port = port.saturating_add(1);
    }
}

pub fn next_spice_port() -> u16 {
    next_free_port(SPICE_PORT_START)
}

pub fn next_rdp_port() -> u16 {
    next_free_port(RDP_PORT_START)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_free_port() {
        let port = next_free_port(15900);
        assert!(port >= 15900);
    }
}
