use toyvec::ToyVec;

fn main() {
    let mut v = ToyVec::new();
    v.push("Java Finch".to_string());
    v.push("Budgerigar".to_string());
    let e = v.get(1);
    assert_eq!(e, Some(&"Budgerigar".to_string()));

    let mut iter = v.iter();
    // v.push("Hill Mynah".to_string());
    assert_eq!(iter.next(), Some(&"Java Finch".to_string()));

    let mut v2 = ToyVec::new();
    v2.push("Hello, ");
    v2.push("World!\n");
    for msg in &v2 {
        print!("{}", msg);
    }
}

// fn error_case() {
//     {
//         let e: Option<&String>;
//         {
//             let mut v = ToyVec::new();
//             v.push("Java Finch".to_string());
//             v.push("Budgerigar".to_string());
//             // 戻り値のlifetime(eのスコープ)がselfのlifetime(vのスコープ)より短くないといけない
//             e = v.get(1);
//         }
//         assert_eq!(e, Some(&"Budgerigar".to_string()));
//     }
// }

// fn error_case() {
//     let mut v = ToyVec::new();
//     v.push(100);
//     let e = v.get(0);
//     v.push(200);
//     assert_eq!(e, Some(&100));
// }
