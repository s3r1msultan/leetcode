/*Two strings, X and Y, are considered similar if either they are identical or we can make them equivalent by swapping at most two letters (in distinct positions) within the string X.

For example, "tars" and "rats" are similar (swapping at positions 0 and 2), and "rats" and "arts" are similar, but "star" is not similar to "tars", "rats", or "arts".

Together, these form two connected groups by similarity: {"tars", "rats", "arts"} and {"star"}.  Notice that "tars" and "arts" are in the same group even though they are not similar.  Formally, each group is such that a word is in the group if and only if it is similar to at least one other word in the group.

We are given a list strs of strings where every string in strs is an anagram of every other string in strs. How many groups are there?



Example 1:

Input: strs = ["tars","rats","arts","star"]
Output: 2

Example 2:

Input: strs = ["omv","ovm"]
Output: 1



Constraints:

1 <= strs.length <= 300
1 <= strs[i].length <= 300
strs[i] consists of lowercase letters only.
All words in strs have the same length and are anagrams of each other.

*/
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

struct DS {
    representatives: HashMap<String, String>,
    size: HashMap<String, i32>
}


impl DS {
    fn new(strings: &Vec<String>) -> Self {
        let mut representatives = HashMap::new();
        let mut size = HashMap::new();
        for string in strings {
            representatives.insert(string.clone(), string.clone());
            size.insert(string.clone(), 1);
        }
        Self {
            representatives,
            size
        }
    }

    fn find(&mut self, x: String) -> String {
        let parent = self.representatives[&x].clone();
        if parent == x {
            return x;
        }

        let root = self.find(parent);
        self.representatives.insert(x, root.clone());
        root
    }

    fn union(&mut self, a: String, b: String) {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return;
        }

        let size_a = self.size[&root_a];
        let size_b = self.size[&root_b];

        match size_a.cmp(&size_b) {
            Ordering::Less => {
                self.representatives.insert(root_a.clone(), root_b.clone());
                self.size.insert(root_b.clone(), size_a + size_b);
                self.size.insert(root_a.clone(), 0);
            }
            Ordering::Equal | Ordering::Greater => {
                self.representatives.insert(root_b.clone(), root_a.clone());
                self.size.insert(root_a.clone(), size_a + size_b);
                self.size.insert(root_b.clone(), 0);
            }
        }
    }
}

pub fn num_similar_groups(strs: Vec<String>) -> i32 {
    let n = strs.len();
    let m = strs[0].len();


    let mut disjoint_set = DS::new(&strs);

    for i in 0..n {
        let first_word = strs[i].as_bytes();

        for j in i+1..n {

            let second_word = strs[j].as_bytes();

            let mut count = 0;
            for k in 0..m {
                if first_word[k] != second_word[k] {
                    count += 1;
                }
            }

            if count == 0 || count == 2 {
                disjoint_set.union(strs[i].clone(), strs[j].clone());
            }
        }
    }

    let mut count = 0;

    let strs= strs.into_iter().collect::<HashSet<_>>();

    for string in strs {
        if string == disjoint_set.find(string.clone()) {
            count += 1;
        }
    }
    count
}