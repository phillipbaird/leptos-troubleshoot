pub mod components;
pub mod constants;
pub mod event_model;
pub mod signals;

use leptos::*;

use components::nodes::*;
use components::selections::*;
use signals::WorkflowSignals;

use crate::event_model::events::Event;
use crate::event_model::types::{CursorId, NodeId, NodeType};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (selected, set_selected) = create_signal(cx, false);

    // Signals updated from domain model.
    let workflow_signals: WorkflowSignals = WorkflowSignals::new(cx);

    // Load demo data.
    let test_node_id = NodeId::new();
    workflow_signals.evolve(Event::NodeCreated {
        id: test_node_id.clone(),
        label: "Some Node".to_owned(),
        node_type: NodeType::Role,
        row: 0,
        col: 1,
    });

    let our_cursor_id = CursorId::new();
    workflow_signals.evolve(Event::CursorCreated {
        id: our_cursor_id.clone(),
        label: "You".to_owned(),
        row: 0,
        col: 1,
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
