use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
struct Parent(usize, Child, Child);

// impl Drop for Parent {
//     fn drop(&mut self) {
//         println!("Dropping {:?}", self)
//     }
// }

#[derive(Clone, Copy, Debug)]
struct Child(usize);

// impl Drop for Child {
//     fn drop(&mut self) {
//         println!("Dropping {:?}", self)
//     }
// }

fn value_scope() {
    let p1 = Parent(1, Child(11), Child(12));

    {
        let p2 = Parent(2, Child(21), Child(22));
        println!("(a) p1: {:?}, p2 {:?}", p1, p2);
    }

    println!("(b) p1: {:?}", p1);
    let p3 = Parent(3, Child(31), Child(32));
    println!("(c) p1: {:?}, p3: {:?}", p1, p3);
}

fn copy_sematics() {
    let p1 = Parent(1, Child(11), Child(12));

    // copy sematics
    let p2 = p1;
    println!("p2: {:?}", p2);
    println!("p1: {:?}", p1);
}

fn move_semantics() {
    fn f1(p: &Parent) {
        println!("p: {:?}", p);
    }

    fn f2(p: &mut Parent) {
        p.0 *= 2;
    }

    let mut p1 = Parent(1, Child(11), Child(12));
    f1(&p1);
    f2(&mut p1);
    println!("p1: {:?}", p1);
}

fn nll() {
    fn process_or_default(key: char, map: &mut HashMap<char, String>) {
        match map.get_mut(&key) {
            Some(value) => value.push_str(", world!"),
            None => {
                map.insert(key, Default::default());
            }
        }
    }

    let mut map = HashMap::new();
    map.insert('h', "hello".to_string());
    process_or_default('h', &mut map);
    println!("{}", map.get(&'h').unwrap());
}

fn main() {
    value_scope();
    copy_sematics();
    move_semantics();
    nll();
}
