#![recursion_limit = "128"]

#[allow(dead_code)]
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::debug!("Hello Yew!");
    //yew::Renderer::<App>::new().render();
}