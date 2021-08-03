use std::{cell::RefCell, collections::HashSet};

fn main() {
    // create RABBITS var in TLS
    thread_local! {
        static RABBITS: RefCell<HashSet< &'static str>> = {
            let rb = ["ロップイヤー", "ダッチ"].iter().cloned().collect();
            RefCell::new(rb)
        }
    };

    // access TLS from main thread
    RABBITS.with(|rb| {
        assert!(rb.borrow().contains("ロップイヤー"));
        rb.borrow_mut().insert("ネザーランド・ドワーフ");
    });

    // access TLS from other thread
    std::thread::spawn(|| RABBITS.with(|rb| rb.borrow_mut().insert("ドワーフホト")))
        .join()
        .expect("Thread Error");

    // access TLS from main thread
    RABBITS.with(|rb| {
        assert!(rb.borrow().contains("ネザーランド・ドワーフ"));
        assert!(!rb.borrow().contains("ドワーフホト"));
    });
}
