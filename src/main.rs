fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    let result = is_odd(1);
    println!("1 is odd: {}", result);
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_number_should_return_true() {
        assert_eq!(is_odd(1), true);
    }

    #[test]
    fn even_number_should_return_false() {
        assert_eq!(is_odd(2), false);
    }
}
