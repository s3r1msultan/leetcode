/*Given two integers left and right that represent the range [left, right], return the bitwise AND of all numbers in this range, inclusive.



Example 1:

Input: left = 5, right = 7
Output: 4

Example 2:

Input: left = 0, right = 0
Output: 0

Example 3:

Input: left = 1, right = 2147483647
Output: 0



Constraints:

0 <= left <= right <= 231 - 1

*/

pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
    let mut result = 0;
    for i in (0..32).rev() {
        match (left >> i & 1, right >> i & 1) {
            (1, 1) => result |= 1 << i,
            (0, 0) => continue,
            (_, _) => return result
        }
    }
    result
}