#![feature(fn_traits)]

use yew::prelude::*;

mod editor;

#[function_component(App)]
fn app() -> Html {
    let content = use_state(|| String::from("hello world\nWhat's up!??"));
    let onchange = {
        let content = content.clone();
        Callback::from(move |c| {
            log::info!("got onchange: {:?}", c);
            content.set(c)
        })
    };
    html! {
        <>
            <h1>{ "Hello World" }</h1>
            <editor::Editor content={"00 00"} {onchange} />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<App>();
}
