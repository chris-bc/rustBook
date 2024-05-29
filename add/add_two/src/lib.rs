pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// Returns the result of adding two to the specified value
/// # Examples
/// ```
/// let a = add_two(3);
/// assert_eq!(a == 5);
pub fn add_two(v: i32) -> i32 {
    v + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
