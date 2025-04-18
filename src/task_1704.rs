/*You are given a string s of even length. Split this string into two halves of equal lengths, and let a be the first half and b be the second half.

Two strings are alike if they have the same number of vowels ('a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'). Notice that s contains uppercase and lowercase letters.

Return true if a and b are alike. Otherwise, return false.



Example 1:

Input: s = "book"
Output: true
Explanation: a = "bo" and b = "ok". a has 1 vowel and b has 1 vowel. Therefore, they are alike.

Example 2:

Input: s = "textbook"
Output: false
Explanation: a = "text" and b = "book". a has 1 vowel whereas b has 2. Therefore, they are not alike.
Notice that the vowel o is counted twice.



Constraints:

2 <= s.length <= 1000
s.length is even.
s consists of uppercase and lowercase letters.

*/

pub fn halves_are_alike(s: String) -> bool {
    use std::collections::HashSet;
    let n = s.len();
    let a = s[..n / 2].as_bytes();
    let b = s[n / 2..].as_bytes();
    let vowels = HashSet::from([b'a', b'e', b'i', b'o', b'u']);

    let mut vowels_1 = 0;
    let mut vowels_2 = 0;

    for i in 0..n / 2 {
        vowels_1 += if vowels.contains(&a[i].to_ascii_lowercase()) { 1 } else { 0 };
    }
    for i in 0..n / 2 {
        vowels_2 += if vowels.contains(&b[i].to_ascii_lowercase()) { 1 } else { 0 };
        ;
    }

    vowels_1 == vowels_2
}