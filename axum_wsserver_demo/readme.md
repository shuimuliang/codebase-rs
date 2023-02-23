# how to test client

## start server
cargo run --bin axum_wsserver_demo

## start client by websocat

```sh
websocat ws://127.0.0.1:3000
```
## start client in Chrome
open index.html
