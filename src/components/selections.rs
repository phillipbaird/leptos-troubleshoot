use leptos::*;

use crate::{components::selection::*, signals::Cursor};

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
