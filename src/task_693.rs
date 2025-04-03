/*Given a positive integer, check whether it has alternating bits: namely, if two adjacent bits will always have different values.



Example 1:

Input: n = 5
Output: true
Explanation: The binary representation of 5 is: 101

Example 2:

Input: n = 7
Output: false
Explanation: The binary representation of 7 is: 111.

Example 3:

Input: n = 11
Output: false
Explanation: The binary representation of 11 is: 1011.



Constraints:

1 <= n <= 231 - 1

*/

pub fn has_alternating_bits(n: i32) -> bool {
    let mut prev_bit = n & 1;
    for i in 1..32 {
        if n >> i == 0 {
            break;
        }

        if n >> i & 1 == prev_bit {
            return false;
        }

        prev_bit = n >> i & 1;
    }
    true
}