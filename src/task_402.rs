// Given string num representing a non-negative integer num, and an integer k, return the smallest possible integer after removing k digits from num.
//
//
//
// Example 1:
//
// Input: num = "1432219", k = 3
// Output: "1219"
// Explanation: Remove the three digits 4, 3, and 2 to form the new number 1219 which is the smallest.
//
// Example 2:
//
// Input: num = "10200", k = 1
// Output: "200"
// Explanation: Remove the leading 1 and the number is 200. Note that the output must not contain leading zeroes.
//
// Example 3:
//
// Input: num = "10", k = 2
// Output: "0"
// Explanation: Remove all the digits from the number and it is left with nothing which is 0.
//
//
//
// Constraints:
//
// 1 <= k <= num.length <= 105
// num consists of only digits.
// num does not have any leading zeros except for the zero itself.
//

pub fn remove_kdigits(num: String, k: i32) -> String {
    let mut k = k;
    let mut stack = vec![];
    for digit in num.chars() {
        while k > 0 && stack.last().map_or(false, |&last| last > digit) {
            stack.pop();
            k-=1;
        }
        stack.push(digit);
    }

    while k > 0 {
        stack.pop();
        k-=1;
    }

    let result = stack.iter().collect::<String>().trim_start_matches("0").to_string();

    if result.is_empty() {
        return "0".to_string()
    }

   result
}

#[cfg(test)]
#[test]
fn test_remove_kdigits() {
    let num = "1432219".to_string();
    let k = 3;
    assert_eq!(remove_kdigits(num, k), "1219".to_string());

    let num = "10200".to_string();
    let k = 1;
    assert_eq!(remove_kdigits(num, k), "200".to_string());

    let num = "10".to_string();
    let k = 2;
    assert_eq!(remove_kdigits(num, k), "0".to_string());
}