use tor::{OwnedTorService, TorService, TorServiceParam};

pub fn create_owned_tor_service() -> OwnedTorService {
    let tor_service = create_tor_service();
    let owned_tor_service = tor_service.into_owned_node().unwrap();
    return owned_tor_service;
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
