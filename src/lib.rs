use std::collections::HashSet;
use itertools::Itertools;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a[&str]) -> HashSet<&'a str> {
    let word_lower_case = word.to_lowercase();
    possible_anagrams
      .iter()
      .filter(|&possible_anagrams| {
        let possible_anagrams_lower = possible_anagrams.to_lowercase();
        possible_anagrams_lower != word_lower_case &&
        possible_anagrams_lower.chars().sorted().eq(word_lower_case.chars().sorted())
      }).copied()
        .collect()
}
