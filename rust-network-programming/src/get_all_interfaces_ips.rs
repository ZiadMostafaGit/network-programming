use if_addrs::get_if_addrs;
use std::io;
use std::net::IpAddr;

pub fn get_interf_and_ip() -> io::Result<()> {
    let interfaes = get_if_addrs()?;

    for interf in interfaes {
        let name = interf.name;
        let ip = interf.addr.ip();
        let iptype = match ip {
            IpAddr::V4(_) => "IPv4",
            IpAddr::V6(_) => "IPv6",
        };

        println!("{}\t{}\t{}", name, iptype, ip);
    }

    Ok(())
}
