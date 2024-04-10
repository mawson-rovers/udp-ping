use std::mem::size_of;

pub const PACKET_LENGTH: usize = 896;
pub const ID_LENGTH: usize = size_of::<u64>();


pub fn generate_packet(count: u64) -> [u8; PACKET_LENGTH] {
    let mut buf = [0u8; PACKET_LENGTH];
    for b in (0..PACKET_LENGTH).step_by(ID_LENGTH) {
        buf[b..b + ID_LENGTH].copy_from_slice(&u64::to_le_bytes(count));
    }
    buf
}

pub fn read_packet_id(buf: &[u8]) -> u64 {
    let mut packet_id = [0u8; ID_LENGTH];
    packet_id.copy_from_slice(&buf[0..ID_LENGTH]);
    let packet_id = u64::from_le_bytes(packet_id);
    packet_id
}
