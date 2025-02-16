/*Given an integer n, return the number of strings of length n that consist only of vowels (a, e, i, o, u) and are lexicographically sorted.

A string s is lexicographically sorted if for all valid i, s[i] is the same as or comes before s[i+1] in the alphabet.



Example 1:

Input: n = 1
Output: 5
Explanation: The 5 sorted strings that consist of vowels only are ["a","e","i","o","u"].

Example 2:

Input: n = 2
Output: 15
Explanation: The 15 sorted strings that consist of vowels only are
["aa","ae","ai","ao","au","ee","ei","eo","eu","ii","io","iu","oo","ou","uu"].
Note that "ea" is not a valid string since 'e' comes after 'a' in the alphabet.

Example 3:

Input: n = 33
Output: 66045



Constraints:

1 <= n <= 50

*/

fn count_vowel_strings(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![vec![0; 5]; n + 1];

    for j in 0..5 {
        dp[1][j] = 1;
    }

    for i in 0..=n {
        for j in 0..5 {
            dp[i][j] = dp[i - 1][0..=j].iter().sum();
        }
    }

    dp[n].iter().sum()
}


#[cfg(test)]
#[test]
fn test_count_vowel_strings() {
    let examples = vec![1, 2, 33];

    for n in examples {
        println!("Input: n = {}", n);
        println!("Output: {}", count_vowel_strings(n));
    }
}