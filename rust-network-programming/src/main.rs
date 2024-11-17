// use if_addrs::get_if_addrs;
use std::io;
// use std::net::IpAddr;
mod get_all_interfaces_ips;
use get_all_interfaces_ips::get_interf_and_ip;
#[cfg(target_os = "windows")]
use winapi::um::winsock2::{WSACleanup, WSAStartup, WSADATA};

// fn get_all_ip_addrs() -> io::Result<()> {
//     // Retrieve all network interfaces
//     let if_addrs = get_if_addrs()?;

//     // Loop through each interface
//     for iface in if_addrs {
//         // Extract the name and IP address type
//         let name = iface.name;
//         let ip = iface.addr.ip();

//         // Check if it is IPv4 or IPv6
//         let ip_type = match ip {
//             IpAddr::V4(_) => "IPv4",
//             IpAddr::V6(_) => "IPv6",
//         };

//         // Print the interface name, address type, and IP address
//         println!("{}\t{}\t{}", name, ip_type, ip);
//     }

//     Ok(())
// }

#[cfg(target_os = "windows")]
fn set_up_socket_for_win_and_linux() -> io::Result<()> {
    let mut data: WSADATA = unsafe { std::mem::zeroed() };
    if unsafe { WSAStartup(0x0202, &mut data) } != 0 {
        eprintln!("Failed to initialize.");
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to initialize.",
        ));
    }

    println!("Ready to use socket API In Windows.");

    unsafe {
        WSACleanup();
    }

    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn set_up_socket_for_win_and_linux() -> io::Result<()> {
    println!("Ready to use socket API from linux.");
    Ok(())
}

fn main() {
    // let _ = set_up_socket_for_win_and_linux();
    let _ = get_interf_and_ip();
}
