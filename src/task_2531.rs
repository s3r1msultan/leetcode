/*You are given two 0-indexed strings word1 and word2.

A move consists of choosing two indices i and j such that 0 <= i < word1.length and 0 <= j < word2.length and swapping word1[i] with word2[j].

Return true if it is possible to get the number of distinct characters in word1 and word2 to be equal with exactly one move. Return false otherwise.



Example 1:

Input: word1 = "ac", word2 = "b"
Output: false
Explanation: Any pair of swaps would yield two distinct characters in the first string, and one in the second string.

Example 2:

Input: word1 = "abcc", word2 = "aab"
Output: true
Explanation: We swap index 2 of the first string with index 0 of the second string. The resulting strings are word1 = "abac" and word2 = "cab", which both have 3 distinct characters.

Example 3:

Input: word1 = "abcde", word2 = "fghij"
Output: true
Explanation: Both resulting strings will have 5 distinct characters, regardless of which indices we swap.



Constraints:

1 <= word1.length, word2.length <= 105
word1 and word2 consist of only lowercase English letters.

*/

pub fn is_it_possible(word1: String, word2: String) -> bool {
    let mut letters_1 = [0; 26];
    let mut letters_2 = [0;26];
    let bytes_1 = word1.as_bytes();
    let bytes_2 = word2.as_bytes();

    for &byte in bytes_1 {
        let i = (byte - b'a') as usize;
        letters_1[i] += 1;
    }

    for &byte in bytes_2 {
        let i = (byte - b'a') as usize;
        letters_2[i] += 1;
    }

    let mut distinct_1 = 0;
    let mut distinct_2 = 0;

    for i in 0..26 {
        if letters_1[i] != 0 {
            distinct_1 += 1;
        }
        if letters_2[i] != 0 {
            distinct_2 += 1;
        }
    }

    for i in 0..26 {
        if (letters_1[i] == 0 && letters_2[i] == 0) || (letters_1[i] > 1 && letters_2[i] > 1) {
            continue;
        }
        if letters_1[i] == 0 && letters_2[i] == 1 {

        } else if letters_1[i] == 1 && letters_2[i] == 0 {

        } else {

        }
    }

    false
}