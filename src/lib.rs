pub mod constants;
pub mod signals;

use leptos::*;

use signals::WorkflowSignals;

use crate::{
    constants::*,
    signals::{Cursor, Event, Node, SelectedNode},
};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (selected, set_selected) = create_signal(cx, false);

    // Signals updated from domain model.
    let workflow_signals: WorkflowSignals = WorkflowSignals::new(cx);

    // Load demo data.
    let test_node_id = uuid::Uuid::new_v4();
    workflow_signals.evolve(Event::NodeCreated {
        id: test_node_id.clone(),
        row: 0,
        col: 1,
    });
    let our_cursor_id = uuid::Uuid::new_v4();
    workflow_signals.evolve(Event::CursorCreated {
        id: our_cursor_id.clone(),
    });

    // ---- Event Listeners ----

    let workflow_signals_clone = workflow_signals.clone();
    window_event_listener_untyped("keydown", move |ev: web_sys::Event| {
        ev.prevent_default();
        if selected.get_untracked() {
            workflow_signals_clone.evolve(Event::NodeDeselected {
                cursor_id: our_cursor_id.clone(),
                node_id: test_node_id.clone(),
            });
            set_selected.set_untracked(false);
        } else {
            workflow_signals_clone.evolve(Event::NodeSelected {
                cursor_id: our_cursor_id.clone(),
                node_id: test_node_id.clone(),
            });
            set_selected.set_untracked(true);
        }
    });

    let nodes = workflow_signals.nodes;
    let cursors = workflow_signals.cursors;

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
            <Selections cursors=cursors/>
          </svg>
        </div>
      </main>
    }
}

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

#[component]
pub fn Selections(cx: Scope, cursors: ReadSignal<Vec<Cursor>>) -> impl IntoView {
    view! { cx,
      <For
        each=cursors
        key= |c| c.id.clone()
        view = move |cx, cursor| {
          let selection_transform = Signal::derive(cx, move ||
            (cursor.selection_transform)().to_transform()
          );

          view! {cx,
            <g transform=selection_transform>
              <Selection selected_nodes=cursor.selected_nodes/>
            </g>
          }
        }
      />
    }
}

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
              </g>
            }
        }
      />

    }
}
