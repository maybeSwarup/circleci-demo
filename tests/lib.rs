#[cfg(test)]
mod tests {
    // Import the factorial function from the factorial_calculator module
    use factorial_calculator::factorial;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }
}
