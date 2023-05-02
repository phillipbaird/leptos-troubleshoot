use leptos::*;

use crate::{constants::*, signals::Node};

#[component]
pub fn Nodes(cx: Scope, nodes: ReadSignal<Vec<Node>>) -> impl IntoView {
    view! { cx,
      <For
        each=nodes
        key= |node| node.id.clone()
        view = move |cx, node| {
            let node_style = "fill-white stroke-gray-300";
            view! {cx,
              <g transform=node.transform>
                <rect width=NODE_WIDTH height=NODE_HEIGHT class=node_style />
                <foreignObject transform=format!("translate({NODE_TEXT_MARGIN}, {NODE_TEXT_MARGIN})") width=(NODE_WIDTH - 10) height=(NODE_HEIGHT - 10)
                  requiredFeatures="http://www.w3.org/TR/SVG11/feature#Extensibility">
                  <p xmlns="http://www.w3.org/1999/xhtml" class="text-lg">{node.label}</p>
               </foreignObject>
              </g>
            }
        }
      />

    }
}
