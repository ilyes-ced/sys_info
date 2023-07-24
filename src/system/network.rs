use std::collections::HashMap;

pub struct Network {
    recv_bytes: u64,
    old_recv_bytes: u64,
    sent_bytes: u64,
    old_sent_bytes: u64,
    recv_packets: u64,
    old_recv_packets: u64,
    sent_packets: u64,
    old_sent_packets: u64,
    recv_errors: u64,
    old_recv_errors: u64,
    /// similar to `recv_errors`
    sent_errors: u64,
    old_sent_errors: u64,
    pub(crate) mac_addr: [u8; 6],
    updated: bool,
}

pub struct Networks(pub HashMap<String, Network>);
