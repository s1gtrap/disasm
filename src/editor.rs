use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub content: String,
    pub onchange: Callback<String>,
}

pub struct Editor(NodeRef);

pub struct NodeIter(std::ops::Range<u32>, web_sys::NodeList);

impl NodeIter {
    fn new(node: &web_sys::Node) -> Self {
        NodeIter(0..node.child_nodes().length(), node.child_nodes())
    }
}

impl Iterator for NodeIter {
    type Item = web_sys::Node;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|i| self.1.item(i).unwrap())
    }
}

fn content(e: &web_sys::Node) -> String {
    let mut s = String::new();

    let l = e.child_nodes();
    for i in 0..l.length() {
        let e = l.item(i).unwrap();
        match e.node_type() {
            web_sys::Node::ELEMENT_NODE if &e.node_name() == "DIV" => {
                s.push('\n');
                s.push_str(&content(&e));
            }
            web_sys::Node::ELEMENT_NODE => {
                s.push_str(&content(&e));
            }
            web_sys::Node::TEXT_NODE => {
                s.push_str(&e.text_content().unwrap());
            }
            _ => todo!(),
        }
    }

    s
}

impl Editor {
    fn get_cursor_pos(&self) -> (usize, usize) {
        let window = web_sys::window().unwrap();
        let sel = window.get_selection().unwrap().unwrap();
        let range = sel.get_range_at(0).unwrap();

        for n in NodeIter::new(&self.0.cast::<web_sys::Node>().unwrap()) {
            log::info!("    {n:?} comparing {:?}", range.start_container().unwrap());
            if n.is_same_node(Some(&range.start_container().unwrap())) {
                log::warn!("found!");
            }
        }

        (0, 0)
    }
}

impl Component for Editor {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        log::info!("Editor::create");
        Editor(NodeRef::default())
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        log::info!("Editor::changed({:?})", ctx.props());

        log::info!("cursor = {:?}", self.get_cursor_pos());

        /*self.0
        .cast::<web_sys::Element>()
        .unwrap()
        .set_text_content(Some(&ctx.props().content));*/

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log::info!("Editor::view");
        let oninput = {
            let node_ref = self.0.clone();
            let onchange = ctx.props().onchange.clone();
            Callback::from(move |_: web_sys::InputEvent| {
                onchange.emit(content(&node_ref.cast::<web_sys::Node>().unwrap()));
            })
        };
        html! {
            <div ref={self.0.clone()} {oninput} style={"font-family:monospace;white-space:pre;min-height:4em;min-width:48em;border:1px solid #000;"} contenteditable="true">
                {&ctx.props().content}
            </div>
        }
    }
}
