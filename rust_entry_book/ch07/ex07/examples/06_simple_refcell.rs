use std::cell::RefCell;

struct A {
    c: char,
    s: String,
}

struct B {
    c: char,
    s: RefCell<String>,
}

fn main() {
    let a = A {
        c: 'a',
        s: "Alex".to_string(),
    };
    let r = &a;
    // r.s.push('a');

    let b = B {
        c: 'a',
        s: RefCell::new("alex".to_string()),
    };
    let rb = &b;
    rb.s.borrow_mut().push('a');
    {
        let rbs = b.s.borrow();
        assert_eq!(*rbs, "alexa");

        // b.s.borrow_mut();

        assert!(b.s.try_borrow_mut().is_err());
    }
    assert!(b.s.try_borrow_mut().is_ok());
}
