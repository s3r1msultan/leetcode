/*Given a string s. Return all the words vertically in the same order in which they appear in s.
Words are returned as a list of strings, complete with spaces when is necessary. (Trailing spaces are not allowed).
Each word would be put on only one column and that in one column there will be only one word.



Example 1:

Input: s = "HOW ARE YOU"
Output: ["HAY","ORO","WEU"]
Explanation: Each word is printed vertically.
"HAY"
"ORO"
"WEU"

Example 2:

Input: s = "TO BE OR NOT TO BE"
Output: ["TBONTB","OEROOE","   T"]
Explanation: Trailing spaces is not allowed.
"TBONTB"
"OEROOE"
"   T"

Example 3:

Input: s = "CONTEST IS COMING"
Output: ["CIC","OSO","N M","T I","E N","S G","T"]



Constraints:

1 <= s.length <= 200
s contains only upper case English letters.
It's guaranteed that there is only one space between 2 words.
*/

pub fn print_vertically(s: String) -> Vec<String> {
    let words = s.split_whitespace().map(|word| word.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let m = words.len();
    let n = words.iter().max_by(|a, b| a.len().cmp(&b.len())).unwrap().len();

    let mut result = vec![vec!['\0'; m]; n];

    for j in 0..m {
        let word = &words[j];
        for i in 0..word.len() {
            result[i][j] = word[i];
        }
    }

    for i in 0..n {
        let chars = &mut result[i];
        while chars.last() == Some(&' ') {
            chars.pop();
        }
    }

    result.into_iter().map(|chars| chars.into_iter().collect::<String>().trim_end().to_string()).collect()
}