use std::str::FromStr;
use std::env::args;
use std::process::exit;
use std::net::IpAddr;

mod scanner;
mod scan_group;

use scanner::Scanner;

fn main() {
    if args().count() == 2 {
        let ip_string = args().nth(1).unwrap();
        let ip_address = IpAddr::from_str(&ip_string);

        match ip_address {
            Ok(address) => scan_ports(address),
            Err(_) => show_error(ip_string),
        }
    } else {
        println!("Usage is: ./radars ip_address");
        exit(1);
    }
}

fn scan_ports(address: IpAddr) {
    Scanner::new(address).scan();
}

fn show_error(address: String) {
    println!("Couldn't parse `{}` as an IP address", address);
    exit(1);
}
