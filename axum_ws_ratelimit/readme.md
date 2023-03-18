# how to test WebSocket rate limit demo

# Project Configuration

- WebSocket Endpoint

```
127.0.0.1:8080
```

## Building Project

```bash
cargo build --release
```

## Starting the Server in Release Mode

```
cargo run --bin axum_ws_ratelimit 
```

## Sending a WebSocket JSON Payload to the Server

1. install **Simple WebSocket Client** in Chrome Browser
   ****[https://chrome.google.com/webstore/detail/simple-websocket-client/pfdhoblngboilpfeibdedpjgfnlcodoo?hl=en](https://chrome.google.com/webstore/detail/simple-websocket-client/pfdhoblngboilpfeibdedpjgfnlcodoo?hl=en)
2. Connect to the WebSocket endpoint.

To connect, simply paste the URL and click the **Connect** button

3. Send the payload in the **Request Form**.

* Message::Text(_)
```
hello from client
```