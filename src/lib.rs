use leptos::*;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: uuid::Uuid,
    pub transform: String,
}

impl Node {
    pub fn new(id: uuid::Uuid) -> Self {
        Node {
            id,
            transform: "translate(100,100)".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SelectedNode {
    pub id: uuid::Uuid,
    pub source_id: uuid::Uuid,
    pub transform: String,
}

impl Into<SelectedNode> for &Node {
    fn into(self) -> SelectedNode {
        SelectedNode {
            id: uuid::Uuid::new_v4(),
            source_id: self.id.clone(),
            transform: self.transform.clone(),
        }
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // ---- Signals ----
    let selected = store_value(cx, false);
    let (nodes, set_nodes) = create_signal(cx, Vec::new() as Vec<Node>);
    let (selected_nodes, set_selected_nodes) = create_signal(cx, Vec::new() as Vec<SelectedNode>);

    // ---- Create single test node. ----
    let test_node_id = uuid::Uuid::new_v4();
    set_nodes.update(|ns| {
        ns.push(Node {
            id: test_node_id,
            transform: "translate(100,100)".to_string(),
        })
    });

    // ---- Keypress Listener / adds removes SelectedNode ----
    window_event_listener_untyped("keydown", move |ev: web_sys::Event| {
        ev.prevent_default();
        if selected() {
            set_selected_nodes.update(|sns| sns.retain(|sn| sn.source_id != test_node_id.clone()));
            selected.set_value(false);
        } else {
            let selected_node = nodes.with_untracked(|ns| {
                ns.iter()
                    .find(|n| n.id == test_node_id.clone())
                    .map(|n| n.into())
                    .expect("Node should exist.")
            });
            set_selected_nodes.update(|sns| sns.push(selected_node));
            selected.set_value(true);
        }
    });

    // ---- View ----

    view! {cx,
      <main>
        <div class="max-w-[100vw]">
          <svg
            class="overflow-scroll"
            width=4000
            height=2000
          >
            <Nodes nodes=nodes/>
            <Selection selected_nodes=selected_nodes/>
          </svg>
        </div>
      </main>
    }
}

#[component]
pub fn Nodes(cx: Scope, nodes: ReadSignal<Vec<Node>>) -> impl IntoView {
    view! { cx,
      <For
        each=nodes
        key= |node| node.id.clone()
        view = move |cx, _node| {
            view! {cx,
                <rect width=200 height=200 class="fill-white stroke-gray-300" />
            }
        }
      />

    }
}

#[component]
pub fn Selection(cx: Scope, selected_nodes: ReadSignal<Vec<SelectedNode>>) -> impl IntoView {
    view! { cx,
      <For
        each=selected_nodes
        key= |selected_node| selected_node.id.clone()
        view = move |cx, _selected_node| {
            view! {cx,
                <rect width=200 height=200 class="fill-purple-300/40 stroke-purple-300/40" />
            }
        }
      />

    }
}
