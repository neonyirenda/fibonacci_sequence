/// Calculate the nth Fibonacci number using recursion
/// 
/// # Arguments
/// * `n` - The position in the Fibonacci sequence (0-based)
/// 
/// # Returns
/// The nth Fibonacci number as a u64
/// 
/// # Examples
/// ```
/// use fibonacci_sequence::fibonacci::fib;
/// 
/// assert_eq!(fib(0), 0);
/// assert_eq!(fib(1), 1);
/// assert_eq!(fib(10), 55);
/// ```
pub fn fib(n: u32) -> u64 {
    if n < 2 {
        n as u64
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

/// Generate a complete Fibonacci sequence up to the nth term
/// 
/// # Arguments
/// * `n` - The number of terms to generate
/// 
/// # Returns
/// A vector containing the Fibonacci sequence from F(0) to F(n)
pub fn generate_sequence(n: u32) -> Vec<u64> {
    let mut sequence = Vec::with_capacity((n + 1) as usize);
    for i in 0..=n {
        sequence.push(fib(i));
    }
    sequence
}

/// Calculate Fibonacci number using memoization for better performance
/// This is more efficient for larger numbers but uses more memory
pub fn fib_memoized(n: u32) -> u64 {
    fn fib_memo_helper(n: u32, memo: &mut std::collections::HashMap<u32, u64>) -> u64 {
        if let Some(&result) = memo.get(&n) {
            return result;
        }
        
        let result = if n < 2 {
            n as u64
        } else {
            fib_memo_helper(n - 1, memo) + fib_memo_helper(n - 2, memo)
        };
        
        memo.insert(n, result);
        result
    }
    
    let mut memo = std::collections::HashMap::new();
    fib_memo_helper(n, &mut memo)
}

/// Generate Fibonacci sequence using iterative approach (most efficient)
pub fn generate_sequence_iterative(n: u32) -> Vec<u64> {
    if n == 0 {
        return vec![0];
    }
    
    let mut sequence = Vec::with_capacity((n + 1) as usize);
    sequence.push(0);
    
    if n >= 1 {
        sequence.push(1);
    }
    
    for i in 2..=n {
        let next = sequence[(i - 1) as usize] + sequence[(i - 2) as usize];
        sequence.push(next);
    }
    
    sequence
}

/// Check if a number is a Fibonacci number
pub fn is_fibonacci_number(num: u64) -> bool {
    // A number is Fibonacci if one of (5*n^2 + 4) or (5*n^2 - 4) is a perfect square
    fn is_perfect_square(n: u64) -> bool {
        let sqrt = (n as f64).sqrt() as u64;
        sqrt * sqrt == n
    }
    
    is_perfect_square(5 * num * num + 4) || is_perfect_square(5 * num * num - 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_basic() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
        assert_eq!(fib(10), 55);
    }

    #[test]
    fn test_generate_sequence() {
        let seq = generate_sequence(5);
        assert_eq!(seq, vec![0, 1, 1, 2, 3, 5]);
    }

    #[test]
    fn test_fib_memoized() {
        assert_eq!(fib_memoized(10), 55);
        assert_eq!(fib_memoized(20), 6765);
    }

    #[test]
    fn test_generate_sequence_iterative() {
        let seq = generate_sequence_iterative(5);
        assert_eq!(seq, vec![0, 1, 1, 2, 3, 5]);
    }

    #[test]
    fn test_is_fibonacci_number() {
        assert!(is_fibonacci_number(0));
        assert!(is_fibonacci_number(1));
        assert!(is_fibonacci_number(2));
        assert!(is_fibonacci_number(3));
        assert!(is_fibonacci_number(5));
        assert!(is_fibonacci_number(8));
        assert!(!is_fibonacci_number(4));
        assert!(!is_fibonacci_number(6));
        assert!(!is_fibonacci_number(7));
    }
}
