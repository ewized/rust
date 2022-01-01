fn _sort(x: &str) -> String {
  let mut chars: Vec<char> = x.chars().collect();
  chars.sort_by(|a, b| a.cmp(b));
  String::from_iter(chars)
}

fn _is_anagram(x: &str, y: &str) -> bool {
  if x.len() != y.len() {
    return false
  }
  _sort(&x).eq(&_sort(&y))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sorts() {
    assert_eq!(_sort("bca"), "abc");
  }

  #[test]
  fn it_works() {
    assert!(_is_anagram("foobar", "foobar"));
    assert!(_is_anagram("barfoo", "foobar"));
    assert!(!_is_anagram("bar", "foobar"));
  }
}
