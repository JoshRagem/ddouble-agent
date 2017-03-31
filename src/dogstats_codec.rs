use std::{env, io};
use std::net::SocketAddr;

use futures::{Future, Poll};
use tokio_core::net::UdpSocket;
use tokio_core::reactor::Core;

pub struct DogstatsCodec;

impl UdpCodec for DogstatsCodec {
    type In = String;
    type Out = String;

    fn decode(&mut self, src: &SocketAddr, buf: &[u8]) -> Result<Option<String>, io::Error> {
        return match str::from_utf8(&buf.as_ref()) {
            Ok(s) => Ok(Some(s.to_string())),
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid string")),
        }
    }

    fn encode(&mut self, msg: String, buf: &mut Vec<u8>) -> io::Result<()> {
        // Reserve enough space for the line
        buf.reserve(msg.len());
        buf.extend(msg.as_bytes());
        Ok(())
    }
}
