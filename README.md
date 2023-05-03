# Leptos Trouble Shooting

Example app to show how the For component can panic.

The following two conditions are are required to cause the panic:
1. The For component is the root element of the view macro.
2. The last item in the For is removed from the collection.

## Requirements
- Assumes you are running Rust nightly.

## Execute

- `trunk serve`

## To Test

1. Running the app your browser will initially display a blank page.
2. Open your browser console so that you can see the panic when it occurs.
3. Click in the browser window to ensure it has keyboard focus.
4. Press any key. The text "Hello World!" will appear.
5. Press another key. The text should be removed but instead we get a panic.