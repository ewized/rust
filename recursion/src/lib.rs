/// Caculates the fibonacci recursivly
/// https://en.wikipedia.org/wiki/Fibonacci_number
pub fn fib(n: usize) -> usize {
  if n <= 1 {
    return n;
  }

  fib(n - 1) + fib(n - 2)
}

/// Caculates the factorial recursivly
/// https://en.wikipedia.org/wiki/Factorial
pub fn fac(n: usize) -> usize {
  if n <= 1 {
    return 1;
  }

  n * fac(n - 1)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn fibonacci() {
    let sequence = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];

    for i in 0..sequence.len() {
      assert_eq!(fib(i), sequence[i]);
    }
  }

  #[test]
  fn factorial() {
    let sequence = [1, 1, 2, 6, 24, 120, 720, 5_040, 40_320, 362_880, 3_628_800];

    for i in 0..sequence.len() {
      assert_eq!(fac(i), sequence[i]);
    }
  }
}
