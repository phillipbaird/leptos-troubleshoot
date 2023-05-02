use leptos::*;

use appname::*;

fn main() {
    mount_to_body(|cx| {
        console_error_panic_hook::set_once();
        _ = console_log::init_with_level(log::Level::Debug);

        log!("csr mode");

        view! { cx,
          <ErrorBoundary
              fallback=move |_, errors| view! { cx, <p class="text-red-500">{format!("{errors:?}")}</p>}
          >
            <App />
          </ErrorBoundary>
        }
    })
}
