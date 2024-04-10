use std::{io, thread};
use std::env::args;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use udp_ping::{CLIENT_PORT, SERVER_PORT, generate_packet, PACKET_LENGTH, read_packet_id};

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();
    let recv_socket = UdpSocket::bind(format!("0.0.0.0:{CLIENT_PORT}"))?;
    let send_socket = UdpSocket::bind("0.0.0.0:0")?; // port assigned by OS
    println!("Sending pings to {}", get_server_addr(&args));
    println!("Listening for replies on {}", recv_socket.local_addr()?);

    let count = 1000;
    let packet_recv = Mutex::new(vec![false; count as usize]);
    let packet_recv = Arc::new(packet_recv);

    let thread_packet_recv = Arc::clone(&packet_recv);
    let recv_thread = thread::spawn(move || {
        loop {
            let mut recv_buf = [0u8; PACKET_LENGTH];
            recv_socket.recv(&mut recv_buf).unwrap();
            let packet_id = read_packet_id(&recv_buf);

            if packet_id <= count {
                let mut packet_recv = thread_packet_recv.lock().unwrap();
                packet_recv[packet_id as usize] = true;
                if packet_recv.iter().all(|b| *b) {
                    break;
                }
            }
        }
    });

    let bar = create_progress_bar(count);
    for count in (0..count).progress_with(bar) {
        let buf = generate_packet(count);
        send_socket.send_to(&buf, get_server_addr(&args))?;
        sleep(Duration::from_millis(5));
    }

    let wait_started = SystemTime::now();
    loop {
        if recv_thread.is_finished() {
            break;
        }
        if SystemTime::now().duration_since(wait_started).unwrap().as_secs() >= 5 {
            break;
        }
        sleep(Duration::from_millis(1));
    }

    let packet_recv = packet_recv.lock().unwrap();
    let received = packet_recv.iter()
        .filter(|&b| *b).count() as u64;
    println!("{} packets transmitted, {} packets received, {:.1}% packet loss",
             count, received, (count - received) as f32 / count as f32 * 100.0);
    Ok(())
}

fn get_server_addr(args: &Vec<String>) -> String {
    format!("{}:{SERVER_PORT}", args.get(1).unwrap_or(&String::from("127.0.0.1")))
}


fn create_progress_bar(count: u64) -> ProgressBar {
    let bar = ProgressBar::new(count);
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("##-"));
    bar.set_message(format!("Transmitting {count} UDP packets"));
    bar
}
