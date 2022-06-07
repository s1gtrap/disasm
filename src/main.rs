#![feature(fn_traits)]

use yew::prelude::*;

mod editor;

#[function_component(App)]
fn app() -> Html {
    let asm = use_state(|| vec![]);
    let err = use_state(|| None);
    let onchange = {
        let asm = asm.clone();
        let err = err.clone();
        Callback::from(move |c: String| {
            match (|| {
                let tokens = dasha::text::tokenize(&c)?;
                dasha::disasm(tokens)
            })() {
                Ok(insts) => {
                    asm.set(insts);
                    err.set(None);
                }
                Err(e) => {
                    asm.set(vec![]);
                    err.set(Some(e));
                }
            }
        })
    };
    html! {
        <>
            <h1>{ "Hello World" }</h1>
            <h5>{ format!("error: {:?}", *err) }</h5>
            <editor::Editor content={"00 00"} {onchange} />
            { format!("{:?}", *asm) }
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<App>();
}
