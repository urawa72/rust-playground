use futures::task;
use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    task::Context,
    time::{Duration, Instant},
};
use std::{task::Poll, thread};

fn main() {
    let mut mini_tokio = MiniTokio::new();

    mini_tokio.spawn(async {
        let when = Instant::now() + Duration::from_millis(10);
        let future = Delay { when };
        let out = future.await;
        assert_eq!(out, "done")
    })
}

struct MiniTokio {
    tasks: VecDeque<Task>,
}


type Task = Pin<Box<dyn Future<Output = ()> + Send>>;

impl MiniTokio {
    fn new() -> Self {
        Self {
            tasks: VecDeque::new(),
        }
    }

    fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.tasks.push_back(Box::pin(future));
    }

    fn run(&mut self) {
        let waker = task::noop_waker();
        let mut cx = Context::from_waker(&waker);
        while let Some(mut task) = self.tasks.pop_front() {
            if task.as_mut().poll(&mut cx).is_pending() {
                self.tasks.push_back(task);
            }
        }
    }
}

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
