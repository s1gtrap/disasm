#![feature(fn_traits)]

use yew::prelude::*;

mod editor;

#[function_component(App)]
fn app() -> Html {
    let asm = use_state(|| vec![]);
    let onchange = {
        let asm = asm.clone();
        Callback::from(move |c: String| {
            let insts = dasha::text::tokenize(&c).unwrap();
            log::info!("got onchange: {:?}", insts);
            asm.set(insts)
        })
    };
    html! {
        <>
            <h1>{ "Hello World" }</h1>
            <editor::Editor content={"00 00"} {onchange} />
            { format!("{:?}", *asm) }
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<App>();
}
