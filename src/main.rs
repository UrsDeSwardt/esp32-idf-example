use esp_idf_svc::{log::EspLogger, sys::link_patches};
use log::info;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    EspLogger::initialize_default();

    loop {
        info!("Hello, world!");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
