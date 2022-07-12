use redis::{self, Client, Commands};
use serde::{Deserialize, Serialize};

fn main() {
    let url = "redis://127.0.0.1:6379/";
    let client = Client::open(url).unwrap();
    let mut con = client.get_connection().unwrap();

    let val = TestValue { id: 1, name: "hoge".to_string()};
    let val = serde_json::to_string(&val).unwrap();

    con.set_ex::<&str, String, ()>("test", val, 10).unwrap();
    let t: bool = con.exists("test").unwrap();
    println!("{:?}", t);

    let res: String = con.get("test").unwrap();
    let val: TestValue = serde_json::from_str(&res).unwrap();

    println!("{:?}", val);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestValue {
    pub id: i16,
    pub name: String
}
