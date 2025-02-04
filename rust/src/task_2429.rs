/*Given two positive integers num1 and num2, find the positive integer x such that:

x has the same number of set bits as num2, and
The value x XOR num1 is minimal.

Note that XOR is the bitwise XOR operation.

Return the integer x. The test cases are generated such that x is uniquely determined.

The number of set bits of an integer is the number of 1's in its binary representation.



Example 1:

Input: num1 = 3, num2 = 5
Output: 3
Explanation:
The binary representations of num1 and num2 are 0011 and 0101, respectively.
The integer 3 has the same number of set bits as num2, and the value 3 XOR 3 = 0 is minimal.

Example 2:

Input: num1 = 1, num2 = 12
Output: 3
Explanation:
The binary representations of num1 and num2 are 0001 and 1100, respectively.
The integer 3 has the same number of set bits as num2, and the value 3 XOR 1 = 2 is minimal.



Constraints:

1 <= num1, num2 <= 109

*/

pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
    let mut result = 0;
    let mut i = 31;
    while result != num1 && result.count_ones() != num2.count_ones() {
        result |= (num1 >> i & 1) << i;
        i -= 1;
    }
    i = 0;
    while result.count_ones() != num2.count_ones() {
        result |= 1 << i;
        i += 1;
    }
    result
}