use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    thread,
    time::{Duration, Instant},
};

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            let waker = cx.waker().clone();
            let when = self.when;
            thread::spawn(move || {
                let now = Instant::now();
                if now < when {
                    thread::sleep(when - now);
                }
                // 続きの処理を行うため紐づいているタスクを実行予約するよう実行器へ合図が送られる
                // ポーリングによって処理を次に進めることができる準備が整ったということを実行器に通知できるようになったときに
                // リソースはwake()を呼び出す
                waker.wake();
            });
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_millis(10);
    let future = Delay { when };

    let out = future.await;
    assert_eq!(out, "done");
}
