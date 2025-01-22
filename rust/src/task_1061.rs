/*You are given two strings of the same length s1 and s2 and a string baseStr.

We say s1[i] and s2[i] are equivalent characters.

For example, if s1 = "abc" and s2 = "cde", then we have 'a' == 'c', 'b' == 'd', and 'c' == 'e'.

Equivalent characters follow the usual rules of any equivalence relation:

Reflexivity: 'a' == 'a'.
Symmetry: 'a' == 'b' implies 'b' == 'a'.
Transitivity: 'a' == 'b' and 'b' == 'c' implies 'a' == 'c'.

For example, given the equivalency information from s1 = "abc" and s2 = "cde", "acd" and "aab" are equivalent strings of baseStr = "eed", and "aab" is the lexicographically smallest equivalent string of baseStr.

Return the lexicographically smallest equivalent string of baseStr by using the equivalency information from s1 and s2.



Example 1:

Input: s1 = "parker", s2 = "morris", baseStr = "parser"
Output: "makkek"
Explanation: Based on the equivalency information in s1 and s2, we can group their characters as [m,p], [a,o], [k,r,s], [e,i].
The characters in each group are equivalent and sorted in lexicographical order.
So the answer is "makkek".

Example 2:

Input: s1 = "hello", s2 = "world", baseStr = "hold"
Output: "hdld"
Explanation: Based on the equivalency information in s1 and s2, we can group their characters as [h,w], [d,e,o], [l,r].
So only the second letter 'o' in baseStr is changed to 'd', the answer is "hdld".

Example 3:

Input: s1 = "leetcode", s2 = "programs", baseStr = "sourcecode"
Output: "aauaaaaada"
Explanation: We group the equivalent characters in s1 and s2 as [a,o,e,r,s,c], [l,p], [g,t] and [d,m], thus all letters in baseStr except 'u' and 'd' are transformed to 'a', the answer is "aauaaaaada".



Constraints:

1 <= s1.length, s2.length, baseStr <= 1000
s1.length == s2.length
s1, s2, and baseStr consist of lowercase English letters.

*/

pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
    struct UnionFind {
        representatives: [u8; 26],
    }

    impl UnionFind {
        fn new() -> Self {
            let mut representatives = [0; 26];
            for i in 0..26 {
                representatives[i] = i as u8 + b'a';
            }
            Self {
                representatives
            }
        }

        fn find(&mut self, x: u8) -> u8 {
            let i = (x - b'a') as usize;
            let parent = self.representatives[i];
            if parent == x {
                return x;
            }

            let root = self.find(parent);
            self.representatives[i] = root;

            root
        }

        fn union(&mut self, a: u8, b: u8) {
            let root_a = self.find(a);
            let root_b = self.find(b);

            if root_a == root_b {
                return;
            }

            if root_a < root_b {
                self.representatives[(root_b - b'a') as usize] = root_a;
            } else {
                self.representatives[(root_a - b'a') as usize] = root_b;
            }
        }
    }
    let mut union_find = UnionFind::new();
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    for (&a, &b) in s1.iter().zip(s2) {
        union_find.union(a, b);
    }

    let mut result = String::new();

    for &byte in base_str.as_bytes() {
        result.push(union_find.find(byte) as char);
    }

    result
}