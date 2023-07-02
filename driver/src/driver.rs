pub struct DriverInitializer;

impl DriverInitializer {
    pub fn http_client() -> reqwest_wasm::Client {
        reqwest_wasm::Client::new()
    }
}