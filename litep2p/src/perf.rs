

const PROTOCOL_NAME: &str = "/litep2p-perf/1.0.0";

pub struct Perf {
    is_server: bool,
    upload_bytes: u64,
    download_bytes: u64,
}

impl Perf {
    pub fn new(is_server: bool, upload_bytes: u64, download_bytes: u64) -> Self {
        Self {
            is_server,
            upload_bytes,
            download_bytes,
        }
    }
}
