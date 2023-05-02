# Leptos Trouble Shooting

Simplified app to demonstrate panic in leptos_dom.

## Requirements

- Tailwind cli needs to be installed as it is called from Trunk hook.
- Assumes you are running Rust nightly.

## Execute

- `trunk serve`

## To Test

1. Your browser should show a rectangle on screen.
2. Open your browser console in order to see the panic.
3. Click in the browser window to ensure it has keyboard focus.
4. Press any key. The rectangle will turn purple as a another rectangle is drawn over the top of the first.
5. Press another key. The purple rectangle should be removed but instead we get a panic.