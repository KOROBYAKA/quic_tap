#[derive(Debug)]
struct TeeUdpSock {
    udp_sock: Arc<dyn AsyncUdpSocket>,
    send_channel: crossbeam_channel::unbounded::sender,
    local_addr: SocketAddr,
    data_buf: vec![u8, 2000],
}

struct CaptureData {
    bufs: Vec<Vec<u8>>,
    meta: Vec<RecvMeta>,
}

impl TeeUdpSock {
    fn new(real_udp_sock: Arc<dyn AsyncUdpSocket>, channel: Sender) -> Result<TeeUdpSock> {
        match real_udp_sock.local_addr() {
            Ok(local_addr) => Ok(TeeUdpSock {
                udp_sock: real_udp_sock,
                send_channel: channel,
                local_addr: local_addr,
            }),
            Err(err) => Err(err),
        }
    }
}

impl AsyncUdpSocket for TeeUdpSock {
    fn create_io_poller(self: Arc<Self>) -> Pin<Box<dyn UdpPoller>> {
        self.udp_sock.create_io_poller()
    }

    fn poll_recv(
        &self,
        cx: &mut Context<'_>,
        bufs: &mut [IoSliceMut<'_>],
        meta: &mut [RecvMeta],
    ) -> Poll<Result<usize>> {
        match self.udp_sock.poll_recv(cx, bufs, meta) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Ready(Ok(n)) => {
                let mut data_cap = CaptureData {
                    bufs: vec![],
                    meta: vec![],
                };
                for i in 0..n {
                    let meta_c = meta[i].clone();
                    data_cap.bufs.push(bufs[i][..meta_c.len].to_vec());
                    data_cap.meta.push(meta_c);
                }

                Poll::Ready(Ok(n))
            }
        }
    }

    fn local_addr(&self) -> Result<SocketAddr> {
        Ok(self.local_addr)
    }

    fn try_send(&self, data_to_transmit: &Transmit<'_>) -> Result<()> {
        self.udp_sock.try_send(data_to_transmit)
    }
}

pub fn key_saver() {}

pub fn packet_saver(rcv_channel: unbounded) {}
