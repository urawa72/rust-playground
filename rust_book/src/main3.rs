fn main() {
    // shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("{}", x);

    // let guess: u32 = "a".parse().expect("Not a number!");
    // println!("{}", guess);

    // tuple
    let x: (i32, f64, u8) = (500, 5.6, 1);
    println!("x tuple {}, {}, {}", x.0, x.1, x.2);

    let (a, b, c) = x;
    println!("x distributed {}, {}, {}", a, b, c);

    // if
    let condition = true;
    let number = if condition { 4 } else { 5 };
    println!("The value of number is: {}", number);

    // array
    let array = [10, 20, 30, 40, 50];
    for el in array.iter() {
        println!("The value is: {}", el);
    }

    c_to_f(28.0);
    println!("Fibonacci: {}", fib(4));
}

fn c_to_f(c: f64) {
    println!("Celsius: {}", c);
    let f = c * 1.8 + 32.0;
    println!("Fahrenheit: {}", f);
}

fn fib(num: usize) -> usize {
    let mut v = vec![0; num + 1];
    v[0] = 1;
    v[1] = 1;
    for i in 1..num {
        v[i + 1] = v[i] + v[i - 1];
    }
    return v[num];
}
