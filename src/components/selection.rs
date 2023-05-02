use leptos::*;

use crate::{constants::*, signals::SelectedNode};

#[component]
pub fn Selection(cx: Scope, selected_nodes: ReadSignal<Vec<SelectedNode>>) -> impl IntoView {
    view! { cx,
      <For
        each=selected_nodes
        key= |selected_node| selected_node.id.clone()
        view = move |cx, selected_node| {
            view! {cx,
              <g transform=selected_node.transform>
                <rect width=NODE_WIDTH height=NODE_HEIGHT class="fill-purple-300/40 stroke-purple-300/40" />
              </g>
            }
        }
      />

    }
}
