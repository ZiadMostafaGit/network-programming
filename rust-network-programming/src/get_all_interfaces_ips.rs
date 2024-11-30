use if_addrs::get_if_addrs;
use std::io;
use std::net::IpAddr;
// first comment with lazy vim

// second cooment
pub fn get_interf_and_ip() -> io::Result<()> {
    let interfaes = get_if_addrs()?;

    //
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

pub fn get_interfaces() -> Option<Vec<String>> {
    match get_if_addrs() {
        Ok(interfaces) => {
            let interface_names: Vec<String> = interfaces
                .into_iter()
                .map(|iface| format!("{} {:#?}", iface.name, iface.addr))
                .collect();

            if interface_names.is_empty() {
                None
            } else {
                Some(interface_names)
            }
        }
        Err(_) => None,
    }
}
