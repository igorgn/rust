use bytes::Bytes;
use mini_redis::{Result, client};
use tokio::sync::{mpsc, oneshot};
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

type Responder<T> = oneshot::Sender<Result<T>>;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get { key, resp } => {
                    let _ = resp.send(client.get(&key).await);
                }
                Set { key, val, resp } => {
                    let _ = resp.send(client.set(&key, val).await);
                }
            };
        }
    });

    let tx2 = tx.clone();
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "hello".to_string(),
            val: Bytes::from("world"),
            resp: resp_tx,
        };
        tx.send(cmd).await.unwrap();
        let result = resp_rx.await;
        println!("Got: {:?}", result);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        tx2.send(Command::Get {
            key: "hello".to_string(),
            resp: resp_tx,
        })
        .await
        .unwrap();

        let result = resp_rx.await;
        println!("Got: {:?}", result);
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
