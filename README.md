To repro:
- Run `cargo run-wasm --package app`.
- Visit http://localhost:8000
- Open the browser console.
- Click on the canvas in the upper lefthand corner (which will not be visible because nothing is rendered).
- Verify order of logs in console.
