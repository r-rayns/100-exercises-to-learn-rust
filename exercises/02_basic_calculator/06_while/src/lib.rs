// Rewrite the factorial function using a `while` loop.
pub fn factorial(n: u32) -> u32 {
    // Init i to the value we wish to find the factorial of
    let mut i = n;
    // Product defaults to 1
    let mut product = 1;
    // Loop calculating the product at each step and subtracting 1 from i
    while i > 0 {
        product *= i;
        i -= 1;
    }
    product
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
