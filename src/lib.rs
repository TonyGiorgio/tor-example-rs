use std::io::prelude::*;
use tor::{OwnedTorService, TorService, TorServiceParam};
use tor_stream::TorStream;

pub fn create_owned_tor_service() -> OwnedTorService {
    let tor_service = create_tor_service();
    let owned_tor_service = tor_service.into_owned_node().unwrap();
    return owned_tor_service;
}

pub fn get_request() -> String {
    let mut stream = TorStream::connect("www.example.com:80").expect("Failed to connect");

    // The stream can be used like a normal TCP stream

    stream
        .write_all(b"GET / HTTP/1.1\r\nConnection: Close\r\nHost: www.example.com\r\n\r\n")
        .expect("Failed to send request");

    // If you want the raw stream, call `into_inner()`

    let mut stream = stream.into_inner();

    let mut buf = String::new();
    stream
        .read_to_string(&mut buf)
        .expect("Failed to read response");

    println!("Server response:\n{}", buf);
    return buf;
}

pub fn create_tor_service() -> TorService {
    let socks_port: u16 = 19054;
    TorServiceParam {
        socks_port: Some(socks_port),
        data_dir: String::from("/tmp/sifir_rs_sdk/"),
        bootstrap_timeout_ms: Some(45000),
    }
    .try_into()
    .unwrap()
}
