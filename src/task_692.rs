/*Given an array of strings words and an integer k, return the k most frequent strings.

Return the answer sorted by the frequency from highest to lowest. Sort the words with the same frequency by their lexicographical order.



Example 1:

Input: words = ["i","love","leetcode","i","love","coding"], k = 2
Output: ["i","love"]
Explanation: "i" and "love" are the two most frequent words.
Note that "i" comes before "love" due to a lower alphabetical order.

Example 2:

Input: words = ["the","day","is","sunny","the","the","the","sunny","is","is"], k = 4
Output: ["the","is","sunny","day"]
Explanation: "the", "is", "sunny" and "day" are the four most frequent words, with the number of occurrence being 4, 3, 2 and 1 respectively.



Constraints:

1 <= words.length <= 500
1 <= words[i].length <= 10
words[i] consists of lowercase English letters.
k is in the range [1, The number of unique words[i]]



Follow-up: Could you solve it in O(n log(k)) time and O(n) extra space?
*/
use std::cmp::Ordering;

struct Word {
    word: String,
    frequency: i32,
}

impl Eq for Word {}

impl PartialEq<Self> for Word {
    fn eq(&self, other: &Self) -> bool {
        self.frequency.eq(&other.frequency)
    }
}

impl PartialOrd<Self> for Word {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.frequency.cmp(&other.frequency) {
            Ordering::Equal => other.word.partial_cmp(&self.word),
            _ => other.frequency.partial_cmp(&self.frequency)
        }
    }
}

impl Ord for Word {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.frequency.cmp(&other.frequency) {
            Ordering::Equal => other.word.cmp(&self.word),
            _ => other.frequency.cmp(&self.frequency)
        }
    }
}
pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
    use std::collections::{BinaryHeap, HashMap};

    let mut map = HashMap::new();
    for word in words {
        *map.entry(word).or_insert(0) += 1;
    }
    let mut heap = BinaryHeap::new();
    for (word, count) in map {
        heap.push(Word {
            word,
            frequency: count,
        });
    }
    let mut result = vec![];
    for _ in 0..k {
        result.push(heap.pop().unwrap().word);
    }
    result
}
