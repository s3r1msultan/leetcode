/*Given two strings str1 and str2, return the shortest string that has both str1 and str2 as subsequences. If there are multiple valid strings, return any of them.

A string s is a subsequence of string t if deleting some number of characters from t (possibly 0) results in the string s.



Example 1:

Input: str1 = "abac", str2 = "cab"
Output: "cabac"
Explanation:
str1 = "abac" is a subsequence of "cabac" because we can delete the first "c".
str2 = "cab" is a subsequence of "cabac" because we can delete the last "ac".
The answer provided is the shortest such string that satisfies these properties.

Example 2:

Input: str1 = "aaaaaaaa", str2 = "aaaaaaaa"
Output: "aaaaaaaa"



Constraints:

1 <= str1.length, str2.length <= 1000
str1 and str2 consist of lowercase English letters.

*/

pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
    let n = str1.len();
    let m = str2.len();
    let s1 = str1.chars().collect::<Vec<char>>();
    let s2 = str2.chars().collect::<Vec<char>>();

    let mut dp = vec![vec![0; m+1]; n+1];
    for i in 0..n {
        for j in 0..m {
            if s1[i] == s2[j] {
                dp[i+1][j+1] = dp[i][j] + 1;
            } else {
                dp[i+1][j+1] = dp[i][j].max(dp[i][j+1]);    
            }
        }
    }

    for i in 0..=n {
        println!("{:?}", dp[i]);
    }

    let mut result = "".to_string();
    result
}