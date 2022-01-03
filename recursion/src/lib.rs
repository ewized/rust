/// Caculates the fibonacci recursivly
/// https://en.wikipedia.org/wiki/Fibonacci_number
fn _fib(n: usize) -> usize {
  if n <= 1 {
    return n;
  }

  _fib(n - 1) + _fib(n - 2)
}

/// Caculates the factorial recursivly
/// https://en.wikipedia.org/wiki/Factorial
fn _fac(n: usize) -> usize {
  if n <= 1 {
    return 1;
  }

  n * _fac(n - 1)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn fibonacci() {
    let sequence = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];

    for i in 0..sequence.len() {
      assert_eq!(_fib(i), sequence[i]);
    }
  }

  #[test]
  fn factorial() {
    let sequence = [1, 1, 2, 6, 24, 120, 720, 5_040, 40_320, 362_880, 3_628_800];

    for i in 0..sequence.len() {
      assert_eq!(_fac(i), sequence[i]);
    }
  }
}
