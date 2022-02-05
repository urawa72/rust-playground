use rstest::rstest;

fn main() {
    println!("{}", finbonacci(4));
}

#[allow(dead_code)]
fn finbonacci(n: usize) -> u32 {
    if n <= 0 {
        return 0;
    }

    let mut v = vec![0u32; n + 2];
    v[0] = 1;
    v[1] = 1;
    for i in 0..n {
        v[i + 2] = v[i + 1] + v[i];
    }
    v[n - 1]
}

#[rstest]
#[case(1, 1)]
#[case(2, 1)]
#[case(3, 2)]
#[case(4, 3)]
fn fibonacci_test1(#[case] input: u32, #[case] expected: u32) {
    assert_eq!(expected, finbonacci(input as usize))
}

#[rstest(input, expected,
    case(0, 0),
    case(1, 1),
    case(2, 1),
    case(3, 2),
    case(4, 3),
    #[should_panic]
    case(0, 1),
)]
fn fibonacci_test2(input: u32, expected: u32) {
    assert_eq!(expected, finbonacci(input as usize))
}
