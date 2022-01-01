use std::collections::HashMap;

fn _sort(x: &str) -> String {
  let mut chars: Vec<char> = x.chars().collect();
  chars.sort_by(|a, b| a.cmp(b));
  String::from_iter(chars)
}

fn _is_anagram(x: &str, y: &str) -> bool {
  if x.len() != y.len() {
    return false;
  }
  _sort(&x).eq(&_sort(&y))
}

fn _group_anagrams(anagrams: Vec<&str>) -> Vec<Vec<String>> {
  let mut lookup: HashMap<String, Vec<String>> = HashMap::new();
  for word in anagrams {
    let sorted = _sort(&word);
    if let Some(arr) = lookup.get_mut(&sorted) {
      arr.push(String::from(word));
    } else {
      lookup.insert(sorted, vec![String::from(word)]);
    }
  }

  //lookup.iter().for_each(|(k, v)| println!("{}: {:?}", k, v));

  lookup.iter().map(|(_, v)| v.clone()).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sorts() {
    assert_eq!(_sort("bca"), "abc");
  }

  #[test]
  fn anagrams() {
    assert!(_is_anagram("foobar", "foobar"));
    assert!(_is_anagram("barfoo", "foobar"));
    assert!(!_is_anagram("bar", "foobar"));
  }

  #[test]
  fn grouping_anagrams() {
    // Used a closure since needed to know the type but also
    // create two seperate empty arrays
    let empty_vec = || -> Vec<&str> { vec![] };
    let empty_vec2 = || -> Vec<Vec<&str>> { vec![] };
    assert_eq!(_group_anagrams(empty_vec()), empty_vec2());

    assert_eq!(
      _group_anagrams(vec!["cat", "tac"]),
      vec![vec!["cat", "tac"]]
    );
    assert_eq!(
      _group_anagrams(vec!["cat", "tac", "bat", "tab"]),
      vec![vec!["cat", "tac"], vec!["bat", "tab"]]
    );
  }
}
