use ulid::Ulid;
use uuid::Uuid;
use std::{thread, time};


fn main() {
    for _ in 1..=15 {
        thread::sleep(time::Duration::from_millis(1));
        let ulid = Ulid::new();
        let uuid: Uuid = ulid.into();
        println!("ulid = {ulid}, uuid = {uuid}");
    }
}
