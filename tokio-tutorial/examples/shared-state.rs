use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use mini_redis::Command::{self, Get, Set};
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

// 多数のスレッド間で共有される
// 複数のタスクから並行的に参照する
type Db = Arc<Mutex<HashMap<String, Bytes>>>;

/// 非同期のコードの中で同期的なMutexを使用するケースは競合が多くなくロックが.awaitをまたいで保持されない場合のみ
/// もし同期Mutexの競合が問題になったとしてもTokioのMutexに切り替えることがベストではない
/// - ステートを管理するための専任タスクを作りメッセージの受け渡しを行う
/// - mutexをシャーディングする
/// - mutexを使わずに済むようにコードを再構築する
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        // Tokioでは何らかの共有ステートへのアクセスを提供する値を指すためにハンドルという用語を用いる
        // ここではHashMapへのハンドルを複製する
        let db = db.clone();
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // ロックを取る
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
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
