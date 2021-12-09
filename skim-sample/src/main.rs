extern crate skim;
use skim::prelude::*;

struct TmpItem {
    title: String,
    tags: Vec<String>,
}

struct MyItem {
    title: String,
    inner: String,
}

impl SkimItem for MyItem {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.inner)
    }

    fn output(&self) -> Cow<str> {
        Cow::Borrowed(&self.title)
    }
}

pub fn main() {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .build()
        .unwrap();

    let tmp = vec![
        TmpItem {
            title: "This is a pen!".to_string(),
            tags: vec!["GitHub".to_string(), "Rust".to_string(), "C++".to_string()],
        },
        TmpItem {
            title: "How to solve this problem?".to_string(),
            tags: vec!["Rust".to_string(), "C++".to_string()],
        },
        TmpItem {
            title: "Hello, World!".to_string(),
            tags: vec!["GitHub".to_string(), "Rust".to_string()],
        },
    ];
    let list: Vec<MyItem> = tmp
        .iter()
        .map(|item| MyItem {
            title: item.title.clone(),
            inner: format!("{}: {:?}", item.title, item.tags),
        })
        .collect();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for item in list {
        let _ = tx_item.send(Arc::new(item));
    }
    drop(tx_item); // so that skim could know when to stop waiting for more items.

    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    for item in selected_items.iter() {
        print!("{}{}", item.output(), "\n");
    }
}
