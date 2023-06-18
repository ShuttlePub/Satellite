#![recursion_limit = "128"]

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);
    let on_click_handler = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    // noinspection ALL
    // language=HTML
    html! {
        <main>
            <button onclick={on_click_handler}>{ "+1" }</button>
            <p>{ *counter }</p>
        </main>
    }
}

#[allow(dead_code)]
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::debug!("Hello Yew!");
    yew::Renderer::<App>::new().render();
}