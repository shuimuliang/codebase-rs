use axum::{
    extract::{
        connect_info::ConnectInfo,
        ws::{Message, WebSocketUpgrade},
        TypedHeader,
    },
    response::IntoResponse,
};

use log::info;
use std::net::SocketAddr;
use std::ops::ControlFlow;

//allows to split the websocket stream into separate TX and RX branches
// use futures::{sink::SinkExt, stream::StreamExt};
use futures_util::stream::{SplitSink, SplitStream};

use futures::{Sink, SinkExt, Stream, StreamExt};
// use axum::extract::ws::CloseFrame;
// use std::borrow::Cow;

pub mod msg;

/// The handler for the HTTP request (this gets called when the HTTP GET lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    // user_agent = None
    // &user_agent = Some(
    //     TypedHeader(
    //         UserAgent(
    //             "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36",
    //         ),
    //     ),
    // )
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };

    info!("`{}` at {} connected.", user_agent, addr.to_string());

    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket<S>(socket: S, who: SocketAddr)
where
    S: Stream<Item = Result<Message, axum::Error>> + Sink<Message> + Send + 'static,
{
    // receive single message from a client (we can either receive or send with socket).
    // this will likely be the Pong for our Ping or a hello message from client.
    // waiting for message from a client will block this task, but will not block other client's
    // connections.

    // while let Some(msg) = socket.recv().await {
    //     if let Ok(msg) = msg {
    //         socket.send(Message::Text(format!("Hello from server {}", 10))).await.unwrap();
    //         if process_message(msg, who).is_break() {
    //             return;
    //         }
    //     } else {
    //         info!("client {} abruptly disconnected", who);
    //         return;
    //     }
    // }

    // for i in 1..5 {
    //     let res = socket.send(Message::Text(format!("Hello from server {}", i))).await;
    //     if res.is_err() {
    //         info!("client {} abruptly disconnected", who);
    //         return;
    //     }
    // }

    let (sender, receiver) = socket.split();

    // This second task will receive messages from client and print them on server console
    // let mut recv_task = tokio::spawn(async move {
    //     let mut cnt = 0;
    //     while let Some(Ok(msg)) = receiver.next().await {
    //         cnt += 1;
    //         info!("received message count: {}", cnt);
    //         // print message and break if instructed to do so
    //         if process_message(msg, who).is_break() {
    //             break;
    //         }
    //     }
    //     cnt
    // });

    // Process each socket concurrently

    // Pseudo-code
    // let join_handle: tokio::task::JoinHandle<Result<i32, JoinError>> = tokio::spawn(async {Ok(5 + 3)});

    let mut recv_task = tokio::spawn(async move { read(receiver, who).await });

    // Spawn a task that will push several messages to the client (does not matter what client does)
    let mut send_task = tokio::spawn(async move { write(sender, who).await });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(a) => info!("{} messages sent to {}", a, who),
                Err(a) => info!("Error sending messages {:?}", a)
            }
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(b) => info!("Received {} messages", b),
                Err(b) => info!("Error receiving messages {:?}", b)
            }
            send_task.abort();
        }
    }

    // returning from the handler closes the websocket connection
    info!("Websocket context {} destroyed", who);
}

/// helper to print contents of messages to stdout. Has special treatment for Close.
fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        // 127.0.0.1:59147 sent str: "'{}'\n"
        Message::Text(t) => {
            info!(">>> {} sent str: {:?}", who, t);
        }
        Message::Close(_c) => {
            info!("client {} close", who);
            return ControlFlow::Break(());
        }
        _ => {}
    }
    ControlFlow::Continue(())
}

async fn read<S>(mut receiver: SplitStream<S>, who: SocketAddr) -> i32
where
    S: Stream<Item = Result<Message, axum::Error>> + Sink<Message> + Send + 'static,
{
    let mut cnt = 0;
    while let Some(Ok(msg)) = receiver.next().await {
        cnt += 1;
        info!("received message count: {}", cnt);
        // print message and break if instructed to do so
        if process_message(msg, who).is_break() {
            break;
        }
    }
    cnt
}

async fn write<S>(mut sender: SplitSink<S, Message>, who: SocketAddr) -> i32
where
    S: Stream<Item = Result<Message, axum::Error>> + Sink<Message> + Send + 'static,
{
    let n_msg = 10;
    for i in 0..n_msg {
        // In case of any websocket error, we exit.
        if sender
            .send(Message::Text(format!("WebSocket Server message {} ...", i)))
            .await
            .is_err()
        {
            return i;
        }

        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    }

    println!("Sending close to {}...", who);
    // if let Err(e) = sender
    //     .send(Message::Close(Some(CloseFrame {
    //         code: axum::extract::ws::close_code::NORMAL,
    //         reason: Cow::from("Goodbye"),
    //     })))
    //     .await
    // {
    //     println!("Could not send Close due to {}, probably it is ok?", e);
    // }

    n_msg
}

#[cfg(test)]
mod tests {
    use tokio::sync::broadcast;

    #[tokio::test]
    async fn test_broadcast() {
        let (sender, mut receiver1) = broadcast::channel(16);
        let mut receiver2 = sender.subscribe();

        tokio::spawn(async move {
            assert_eq!(receiver1.recv().await.unwrap(), 10);
            assert_eq!(receiver1.recv().await.unwrap(), 20);
        });

        tokio::spawn(async move {
            assert_eq!(receiver2.recv().await.unwrap(), 10);
            assert_eq!(receiver2.recv().await.unwrap(), 20);
        });

        sender.send(10).unwrap();
        sender.send(20).unwrap();
    }

    use anyhow;
    use axum::extract::ws::Message;
    use fake_socket::{create_fake_connection, FakeClient};
    use std::net::SocketAddr;

    use super::handle_socket;
    use crate::msg;

    async fn prepare_connections() -> anyhow::Result<FakeClient<Message>> {
        let (mut client, socket) = create_fake_connection();
        let who = SocketAddr::from(([127, 0, 0, 1], 10000));

        tokio::spawn(async move { handle_socket(socket, who).await });

        let raw_msg = msg::Msg::new(msg::ActionType::Join);
        let json = serde_json::to_string(&raw_msg).unwrap();
        let ws_msg = Message::Text(json);

        client.send(ws_msg)?;
        while let Some(Message::Text(msg)) = client.recv().await {
            dbg!(&msg);
        }

        Ok(client)
    }

    #[tokio::test]
    async fn test_server_echo() -> anyhow::Result<()> {
        let _client = prepare_connections().await?;
        Ok(())
    }
}
