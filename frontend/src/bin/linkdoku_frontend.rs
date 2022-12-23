use frontend::App;

fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    yew::Renderer::<App>::new().hydrate();
}
