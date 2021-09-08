pub mod ut;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub fn add3(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add3() {
        assert_eq!(add3(2, 3), 5);
    }
}
