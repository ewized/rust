
fn sort(x: &str) -> String {
  let mut chars: Vec<char> = x.chars().collect();
  chars.sort_by(|a, b| a.cmp(b));
  String::from_iter(chars)
}

fn is_anagram(x: &str, y: &str) -> bool {
  if x.len() != y.len() {
    return false
  }
  sort(&x).eq(&sort(&y))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sorts() {
    assert_eq!(sort("bca"), "abc");
  }

  #[test]
  fn it_works() {
    assert!(is_anagram("foobar", "foobar"));
    assert!(is_anagram("barfoo", "foobar"));
    assert!(!is_anagram("bar", "foobar"));
  }
}
