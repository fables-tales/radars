use std::thread::{spawn, JoinHandle};
use std::io::{self, Write};
use std::net::{SocketAddr, TcpStream};

pub type PortNumber=u16;

fn port_is_open(addr: &SocketAddr) -> bool {
    let conn = TcpStream::connect(addr);
    conn.is_ok()
}

pub struct ScanGroup {
    addrs: Vec<SocketAddr>
}

impl ScanGroup {
    pub fn new(addrs: Vec<SocketAddr>) -> ScanGroup {
        ScanGroup {
            addrs: addrs
        }
    }

    pub fn dispatch(self) -> JoinHandle<Vec<PortNumber>> {
        spawn(move || self.perform_scan())
    }

    fn perform_scan(self) -> Vec<PortNumber> {
        self.addrs.into_iter().filter(|addr| {
            report_progress();
            port_is_open(addr)
        }).map(|addr| addr.port()).collect()
    }

    fn report_progress() {
        print!(".");
        io::stdout().flush().unwrap();
    }
}
