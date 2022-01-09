use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
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

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    // 32のキャパシティをもったチャネル
    let (tx, mut rx) = mpsc::channel(32);
    // 複数のタスクからの送信
    let tx2 = tx.clone();
    let tx3 = tx.clone();

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;
                    let _ = resp.send(res);
                }
                Command::Set { key, val, resp } => {
                    let res = client.set(&key, val.into()).await;
                    let _ = resp.send(res);
                }
            }
        }
    });

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        tx.send(Command::Get {
            key: "hello".to_string(),
            resp: resp_tx,
        })
        .await
        .unwrap();
        let res = resp_rx.await;
        println!("t1 GOT = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        tx2.send(Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx,
        })
        .await
        .unwrap();
        let res = resp_rx.await;
        println!("t2 GOT = {:?}", res);
    });

    let t3 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        tx3.send(Command::Get {
            key: "foo".to_string(),
            resp: resp_tx,
        })
        .await
        .unwrap();
        let res = resp_rx.await;
        println!("t3 GOT = {:?}", res);
    });

    // joinハンドルをawait
    t1.await.unwrap();
    t2.await.unwrap();
    t3.await.unwrap();
    manager.await.unwrap();
}
