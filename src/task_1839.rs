/*A string is considered beautiful if it satisfies the following conditions:

Each of the 5 English vowels ('a', 'e', 'i', 'o', 'u') must appear at least once in it.
The letters must be sorted in alphabetical order (i.e. all 'a's before 'e's, all 'e's before 'i's, etc.).

For example, strings "aeiou" and "aaaaaaeiiiioou" are considered beautiful, but "uaeio", "aeoiu", and "aaaeeeooo" are not beautiful.

Given a string word consisting of English vowels, return the length of the longest beautiful substring of word. If no such substring exists, return 0.

A substring is a contiguous sequence of characters in a string.



Example 1:

Input: word = "aeiaaioaaaaeiiiiouuuooaauuaeiu"
Output: 13
Explanation: The longest beautiful substring in word is "aaaaeiiiiouuu" of length 13.

Example 2:

Input: word = "aeeeiiiioooauuuaeiou"
Output: 5
Explanation: The longest beautiful substring in word is "aeiou" of length 5.

Example 3:

Input: word = "a"
Output: 0
Explanation: There is no beautiful substring, so return 0.



Constraints:

1 <= word.length <= 5 * 105
word consists of characters 'a', 'e', 'i', 'o', and 'u'.

*/

pub fn longest_beautiful_substring(word: String) -> i32 {
    let n = word.len();
    let bytes = word.as_bytes();

    let mut set = std::collections::HashSet::new();
    set.insert(bytes[0]);
    let mut max = 0;
    let mut left = 0;
    for right in 1..n {
        if bytes[right - 1] > bytes[right] {
            left = right;
            set.clear();
            set.insert(bytes[right]);
            continue;
        }

        set.insert(bytes[right]);

        if set.len() == 5 {
            max = max.max(right - left + 1);
        }
    }
    max as i32
}