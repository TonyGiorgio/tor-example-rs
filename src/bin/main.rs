use tor_example_rs::create_owned_tor_service;

pub fn main() {
    let _tor_service = create_owned_tor_service();

    println!("Tor Created!!\n Socks5 Proxy: 127.0.0.1:{}\n", 19054);
}
