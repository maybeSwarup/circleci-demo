/// Calculate the factorial of a given number.
///
/// # Arguments
///
/// * `n` - The number whose factorial is to be calculated
///
/// # Examples
///
/// ```
/// use factorial_calculator::factorial;
///
/// let result = factorial(5);
/// assert_eq!(result, 120);
/// ```
pub fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}
