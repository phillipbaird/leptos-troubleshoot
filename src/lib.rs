use leptos::*;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: usize,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let selected = store_value(cx, false);
    let (nodes, set_nodes) = create_signal(cx, Vec::new() as Vec<Node>);

    let test_node_id = 1;

    // ---- Keypress Listener - adds/removes a node. ----
    window_event_listener_untyped("keydown", move |ev: web_sys::Event| {
        ev.prevent_default();
        if selected() {
            set_nodes.update(|sns| sns.retain(|sn| sn.id != test_node_id));
            selected.set_value(false);
        } else {
            set_nodes.update(|sns| sns.push(Node { id: test_node_id }));
            selected.set_value(true);
        }
    });

    view! {cx,
      // <div> - adding this div fixes the failing For.
      <For
        each=nodes
        key= |node| node.id.clone()
        view = move |cx, _node| {
            view! {cx,
              <p>"Hello World!"</p>
            }
        }
      />
      // </div>
    }
}
