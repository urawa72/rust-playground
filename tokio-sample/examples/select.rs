use std::time::Duration;

use tokio::time::delay_for;

#[tokio::main]
async fn main() {
    let delay = delay_for(Duration::from_millis(50));
    let fut = async {
        delay_for(Duration::from_millis(20)).await;
        1
    };

    tokio::select! {
        _ = delay => {
            println!("timeoute");
        }
        v = fut => {
            println!("result: {}", v);
        }
    }

    let delay = delay_for(Duration::from_millis(50));
    let fut = async {
        delay_for(Duration::from_millis(100)).await;
        2
    };

    tokio::select! {
        _ = delay => {
            println!("timeoute");
        }

        v = fut => {
            println!("result: {}", v);
        }
    }
}
