use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
struct Subnet {
    network_address: Ipv4Addr,
    subnet_mask: Ipv4Addr,
}

impl Subnet {
    fn new(network_address: Ipv4Addr, subnet_mask: Ipv4Addr) -> Subnet {
        Subnet {
            network_address,
            subnet_mask,
        }
    }

    fn get_network_address(&self) -> Ipv4Addr {
        let network_address_u32 = u32::from(self.network_address);
        let subnet_mask_u32 = u32::from(self.subnet_mask);
        Ipv4Addr::from(network_address_u32 & subnet_mask_u32)
    }

    fn get_broadcast_address(&self) -> Ipv4Addr {
        let network_address_u32 = u32::from(self.network_address);
        let subnet_mask_u32 = u32::from(self.subnet_mask);
        let inverted_subnet_mask_u32 = !subnet_mask_u32;
        Ipv4Addr::from(network_address_u32 | inverted_subnet_mask_u32)
    }

    fn get_usable_ip_range(&self) -> (Ipv4Addr, Ipv4Addr) {
        let network_address_u32 = u32::from(self.get_network_address());
        let broadcast_address_u32 = u32::from(self.get_broadcast_address());
        let first_usable_ip_u32 = network_address_u32 + 1;
        let last_usable_ip_u32 = broadcast_address_u32 - 1;
        (
            Ipv4Addr::from(first_usable_ip_u32),
            Ipv4Addr::from(last_usable_ip_u32),
        )
    }
}

fn main() {
    let subnet = Subnet::new(Ipv4Addr::new(192, 168, 1, 0), Ipv4Addr::new(255, 255, 255, 0));
    
    println!("Network Address: {}", subnet.get_network_address());
    println!("Broadcast Address: {}", subnet.get_broadcast_address());
    
    let (first_usable_ip, last_usable_ip) = subnet.get_usable_ip_range();
    println!("Usable IP Range: {} - {}", first_usable_ip, last_usable_ip);
}