use std::net::{IpAddr, SocketAddr};
use std::ops::Range;
use std::thread::{self, JoinHandle};

use scan_group::{ScanGroup, PortNumber};

pub struct Scanner {
    target: IpAddr
}

fn ports() -> Range<PortNumber> {
    (1..65535)
}

fn n_threads() -> u32 {
    1000
}

fn await_results<T>(handles: Vec<JoinHandle<T>>) -> Vec<thread::Result<T>> {
    handles.into_iter().map(|h| h.join()).collect()
}

impl Scanner {
    pub fn new(target: IpAddr) -> Scanner {
        Scanner {
            target: target
        }
    }

    pub fn scan(self) {
        let handles = self.build_scan_groups();
        self.print_handle_results(handles);
    }

    fn build_scan_groups(&self) -> Vec<JoinHandle<Vec<PortNumber>>> {
        let addrs = self.socket_addrs();
        let chunks = addrs.chunks(addrs.len() / n_threads() as usize);

        chunks.into_iter().map(|chunk| {
            ScanGroup::new(chunk.to_vec()).dispatch()
        }).collect()
    }

    fn print_handle_results(&self, handles: Vec<JoinHandle<Vec<PortNumber>>>) {
        let results: Vec<_> = await_results(handles);

        for result in results {
            let open_ports = result;
            match open_ports {
                Ok(x) => for open_port in x {
                    println!("{} is open", open_port);
                },
                Err(_) => println!("Error: joining a thread failed"),
            }
        }
    }

    fn socket_addrs(&self) -> Vec<SocketAddr> {
        ports().into_iter().map(|port_number| {
            SocketAddr::new(self.target, port_number)
        }).collect()
    }
}
