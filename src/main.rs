extern crate ddouble;
extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate tokio_core;

use std::net::SocketAddr;

use tokio_core::net::UdpSocket;
use tokio_core::reactor::Core;

use ddouble::dogstats_codec::DogstatsCodec;

fn main() {
    let addr = "0.0.0.0:8125";
    let addr = addr.parse::<SocketAddr>().unwrap();

    // Create the event loop that will drive this server, and also bind the
    // socket we'll be listening to.
    let l = Core::new().unwrap();
    let handle = l.handle();
    let socket = UdpSocket::bind(&addr, &handle).unwrap();
    println!("Listening on: {}", addr);
    socket.framed(DogstatsCodec);
}
