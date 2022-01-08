use std::collections::HashMap;

use mini_redis::Command::{self, Get, Set};
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

/// Tokioのタスクは非同期のグリーンスレッドで、asyncブロックをtokio::spawnに渡すことで作られる
/// spawnされるタスクはタスク外で所有されているデータへの参照を含んではならない('static境界)
/// タスクは必ずSendトレイトを実装していなければならない(Send境界)
/// .awaitによってタスクが一時的に中断されているときにTokioランタイムがそのタスクを別スレッドに移動することができるようにするため
/// Rustではほとんどの型はSendだがRc<T>など一部例外がある
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            // Rc<T>はSend未実装なのでエラーになる
            // let rc = Rc::new("hello");
            process(socket).await;

        });
    }
}

async fn process(socket: TcpStream) {
    let mut db = HashMap::new();
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}
