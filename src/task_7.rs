// Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.
//
// Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
//
//
//
// Example 1:
//
// Input: x = 123
// Output: 321
//
// Example 2:
//
// Input: x = -123
// Output: -321
//
// Example 3:
//
// Input: x = 120
// Output: 21
//
//
//
// Constraints:
//
// -231 <= x <= 231 - 1
//
// pub fn reverse(x: i32) -> i32 {
//
//     let mut x = x;
//     let mut ans: i64 = 0;
//     if x/10 == 0 {
//         return x;
//     }
//     while(x != 0) {
//         ans = ans*10 + x as i64 % 10i64;
//         if ans > i32::MAX as i64 || ans < i32::MIN as i64{
//             return 0;
//         }
//         x /= 10;
//     }
//     ans as i32
// }

pub fn reverse(x: i32) -> i32 {
    x.signum() * x.abs().to_string().chars().rev().collect::<String>().parse::<i32>().unwrap_or(0)
}


#[cfg(test)]

#[test]
fn test_reverse() {
    let x =  123;
    assert_eq!(reverse(x), 321);
    let x = -123;
    assert_eq!(reverse(x), -321);
    let x= 120;
    assert_eq!(reverse(x), 21);
}