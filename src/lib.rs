use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Debug)]
pub struct Message {
    text: Arc<String>,
    sender_addr: SocketAddr,
    size: usize,
}

impl Message {
    pub fn new(size: usize, text: Arc<String>, sender_addr: SocketAddr) -> Message {
        Message {
            size: size,
            text: text,
            sender_addr: sender_addr,
        }
    }

    pub fn get_text(&self) -> String {
        let str = &*(self.text);
        str.clone()
    }

    pub fn get_sender_addr(&self) -> SocketAddr {
        self.sender_addr
    }

    pub fn raw(&self) -> Vec<u8> {
        let mut buff = self.get_text().into_bytes();
        buff.resize(self.size, 0);
        buff
    }
}
