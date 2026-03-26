use std::collections::{HashSet, HashMap};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let str_lower = word.to_lowercase();
    let str_counts = char_counts(&str_lower);

    possible_anagrams
            .iter()
            .filter(|a| {
                let a_low = a.to_lowercase();

                // 1. 不能和自己相等
                if a_low == str_lower{
                    return false;
                }

                // 2. 字母数量一致
                 char_counts(&a_low) == str_counts
            })
            // .cloned()
            .copied()
            .collect::<HashSet::<& str>>()
}

fn char_counts(s: &str)-> HashMap<char, u32>{
    let mut counts = HashMap::new();
    for c in s.chars(){
        *counts.entry(c).or_insert(0) += 1;
    }
    counts
}