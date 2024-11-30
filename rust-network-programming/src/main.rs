use if_addrs::get_if_addrs;
use std::io;
// use std::net::IpAddr;
mod get_all_interfaces_ips;
use chrono::{Local, TimeZone};
use get_all_interfaces_ips::{get_interf_and_ip, get_interfaces};
use std::io::Write;
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    // Start a TCP listener on port 8080
    let listener = TcpListener::bind("0.0.0.0:5000")?;
    println!("The server is running on 0.0.0.0:5000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection received!");
                handle_income(stream);
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_income(mut stream: TcpStream) {
    let now = Local::now();

    // Create an HTTP response
    let body = format!(
        "Hi iam Ziad Mostafa and this is my TCP Socket\nCurrent time: {}",
        now.format("%Y-%m-%d %H:%M:%S")
    );
    let response = format!(
        "HTTP/1.1 200 OK\r\n\
        Content-Type: text/plain; charset=UTF-8\r\n\
        Content-Length: {}\r\n\
        Connection: close\r\n\r\n\
        {}",
        body.len(),
        body
    );

    // Send the response to the client
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to send the response: {}", e);
    }
}
