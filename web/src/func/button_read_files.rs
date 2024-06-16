use log::info;
use yew::prelude::*;
use web_sys::window;


#[function_component(ButtonReadFiles)]
pub fn button_read_files() -> Html {
    let onclick = Callback::from(|_| {
        info!("Button clicked!");
        
        

        if let Some(win) = window() {
            if let Err(err) = win.alert_with_message("Логи прочитаны!") {
                eprintln!("Failed to show alert: {:?}", err);
            }
        }
    });

    html! {
        <button onclick={onclick}>{"Прочитать логи"}</button>
    }
}

