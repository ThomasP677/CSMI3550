
extern crate if_addrs;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn main() {
    let interfaces = if_addrs::get_if_addrs().expect("Failed to get network interfaces");
    for interface in interfaces {
        if let Some(ip) = interface.ip() {
            if !ip.is_loopback() && !ip.is_link_local() {
                match ip {
                    IpAddr::V4(ipv4) => {
                        println!("Local IPv4 Address: {}", ipv4);
                    }
                    IpAddr::V6(ipv6) => {
                        println!("Local IPv6 Address: {}", ipv6);
                    }
                }
            }
        }
    }
