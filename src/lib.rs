use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! { "hello world" }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
