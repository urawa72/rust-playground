use std::{
    sync::{Arc, Mutex, MutexGuard},
    thread::{self, JoinHandle},
};

fn thread_test() {
    let handle: JoinHandle<()> = thread::spawn(|| {
        println!("Hello, from thread.");
    });
    handle.join().unwrap();
}

fn multi_thread_test() {
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for i in 1..=10 {
        let test = String::from("test");
        handles.push(thread::spawn(move || {
            println!("Hello, from thread. {}, {}", i, test);
        }))
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn shared_memory_test() {
    let shared: Arc<Vec<i32>> = Arc::new(vec![1, 2, 3]);
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for i in 1..=10 {
        let s_ref: Arc<Vec<i32>> = shared.clone();
        handles.push(thread::spawn(move || {
            println!("Hello, from thread. {}, {:?}", i, s_ref);
        }))
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn shared_counter() {
    let shared_counter: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for i in 1..=10 {
        let shared_counter_ref: Arc<Mutex<u32>> = shared_counter.clone();
        handles.push(thread::spawn(move || {
            let mut c: MutexGuard<u32> = shared_counter_ref.lock().unwrap();
            *c += 1;
            println!("Hello, from thread. {}, {}", i, c);
        }))
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", *shared_counter.lock().unwrap());
}

fn main() {
    thread_test();
    multi_thread_test();
    shared_memory_test();
    shared_counter();
}
