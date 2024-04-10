use std::io;
use std::io::{stdout, Write};
use std::net::UdpSocket;
use udp_ping::{ID_LENGTH, PACKET_LENGTH, read_packet_id};

fn main() -> io::Result<()> {
    let recv_socket = UdpSocket::bind("0.0.0.0:14000")?;
    let send_socket = UdpSocket::bind("0.0.0.0:14001")?;

    let mut count = 0u64;
    loop {
        let mut buf = [0; PACKET_LENGTH];
        recv_socket.recv(&mut buf)?;
        let packet_id = read_packet_id(&mut buf);

        let mut buf = [0; PACKET_LENGTH];
        buf[0..ID_LENGTH].copy_from_slice(&u64::to_ne_bytes(packet_id));
        send_socket.send_to(&buf, "127.0.0.1:12000")?;

        count += 1;
        if count % 80 == 0 {
            println!(".");
        } else {
            print!(".");
            stdout().flush()?
        }
    }
}
