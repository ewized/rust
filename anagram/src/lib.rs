use std::collections::HashMap;

fn sort(x: &str) -> String {
  let mut chars: Vec<char> = x.chars().collect();
  chars.sort_by(|a, b| a.cmp(b));

  String::from_iter(chars)
}

pub fn is_anagram(x: &str, y: &str) -> bool {
  if x.len() != y.len() {
    return false;
  }

  sort(&x) == sort(&y)
}

pub fn group_anagrams(anagrams: Vec<&str>) -> Vec<Vec<String>> {
  let mut lookup: HashMap<String, Vec<String>> = HashMap::new();

  for word in anagrams {
    let sorted = sort(&word);
    let string_word = String::from(word);

    match lookup.get_mut(&sorted) {
      Some(arr) => {
        arr.push(string_word);
      }
      _ => {
        lookup.insert(sorted, vec![string_word]);
      }
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
    assert_eq!(sort("bca"), "abc");
  }

  #[test]
  fn anagrams() {
    assert!(is_anagram("foobar", "foobar"));
    assert!(is_anagram("barfoo", "foobar"));
    assert!(!is_anagram("bar", "foobar"));
  }

  #[test]
  fn grouping_anagrams() {
    assert_eq!(
      group_anagrams(vec![] as Vec<&str>),
      vec![] as Vec<Vec<&str>>
    );

    assert_eq!(
      group_anagrams(vec!["cat", "tac"]),
      vec![vec!["cat", "tac"]]
    );
    // order is not guaranteed in the outer Vec as we use a HashMap internally so we sort it for the unit test
    let mut grouped = group_anagrams(vec!["cat", "tac", "bat", "tab"]);
    grouped.sort();
    assert_eq!(grouped, vec![vec!["bat", "tab"], vec!["cat", "tac"]]);
  }
}
