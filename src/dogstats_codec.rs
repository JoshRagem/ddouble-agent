use std::{io, str};
use std::net::SocketAddr;

use tokio_core::net::UdpCodec;

pub struct DogstatsCodec;

impl UdpCodec for DogstatsCodec {
    type In = (SocketAddr, String);
    type Out = (SocketAddr, String);

    fn decode(&mut self, addr: &SocketAddr, buf: &[u8]) -> io::Result<Self::In> {
        return match str::from_utf8(&buf.as_ref()) {
            Ok(s) => Ok((*addr, s.to_string())),
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid string")),
        }
    }

    fn encode(&mut self, (addr, msg): Self::Out, buf: &mut Vec<u8>) -> SocketAddr {
        buf.reserve(msg.len());
        buf.extend(msg.as_bytes());
        addr
    }
}
