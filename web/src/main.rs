use yew::prelude::*;

mod func;
use crate::func::button_read_files::ButtonReadFiles;

#[function_component(App)]
fn app() -> Html {
    html! {
    <>
        <h1>{ "Анализ логов почтового сервера" }</h1>
        <div>
            <ButtonReadFiles />
        </div>
    </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
