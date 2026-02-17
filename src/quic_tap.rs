#[derive(Debug)]
struct TeeUdpSock {
    udp_sock: Arc<dyn AsyncUdpSocket>,
    send_channel: crossbeam_channel::unbounded::sender,
    sock_id: u32,
}

pub fn key_saver() {}

pub fn packet_saver(rcv_channel: unbounded) {}
