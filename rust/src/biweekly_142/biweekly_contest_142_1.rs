/*Alice is attempting to type a specific string on her computer. However, she tends to be clumsy and may press a key for too long, resulting in a character being typed multiple times.

Although Alice tried to focus on her typing, she is aware that she may still have done this at most once.

You are given a string word, which represents the final output displayed on Alice's screen.

Return the total number of possible original strings that Alice might have intended to type.



Example 1:

Input: word = "abbcccc"

Output: 5

Explanation:

The possible strings are: "abbcccc", "abbccc", "abbcc", "abbc", and "abcccc".

Example 2:

Input: word = "abcd"

Output: 1

Explanation:

The only possible string is "abcd".

Example 3:

Input: word = "aaaa"

Output: 4



Constraints:

1 <= word.length <= 100
word consists only of lowercase English letters.

Note: Please do not copy the description during the contest to maintain the integrity of your submissions.*/


pub fn possible_string_count(word: String) -> i32 {
    let chars = word.chars().collect::<Vec<char>>();
    let mut groups = vec![];
    let mut i = 0;
    while i < chars.len() {
        let mut j = i + 1;
        while j < chars.len() && chars[i] == chars[j] {
            j += 1;
        }
        groups.push(j - i);
        i = j;
    }

    let mut count = 1;

    for letter_count in groups {
        if letter_count > 1 {
            count += letter_count as i32;
        }
    }

    count
}