use std::env::args;
use std::io;
use std::io::{stdout, Write};
use std::net::UdpSocket;

use udp_ping::{ID_LENGTH, PACKET_LENGTH, read_packet_id, CLIENT_PORT, SERVER_PORT};

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();
    let recv_socket = UdpSocket::bind(format!("0.0.0.0:{SERVER_PORT}"))?;
    let send_socket = UdpSocket::bind("0.0.0.0:0")?; // port assigned by OS
    println!("Listening for pings on {}", recv_socket.local_addr()?);
    println!("Replies will be sent to {}", get_client_addr(&args));

    let mut count = 0u64;
    loop {
        let mut buf = [0; PACKET_LENGTH];
        recv_socket.recv(&mut buf)?;
        let packet_id = read_packet_id(&mut buf);

        let mut buf = [0; PACKET_LENGTH];
        buf[0..ID_LENGTH].copy_from_slice(&u64::to_ne_bytes(packet_id));
        send_socket.send_to(&buf, get_client_addr(&args))?;

        count += 1;
        if count % 80 == 0 {
            println!(".");
        } else {
            print!(".");
            stdout().flush()?
        }
    }
}

fn get_client_addr(args: &Vec<String>) -> String {
    format!("{}:{CLIENT_PORT}", args.get(1).unwrap_or(&String::from("127.0.0.1")))
}
