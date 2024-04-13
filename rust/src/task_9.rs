// Given an integer x, return true if x is a palindrome, and false otherwise.
//
//
//
// Example 1:
//
// Input: x = 121
// Output: true
// Explanation: 121 reads as 121 from left to right and from right to left.
//
// Example 2:
//
// Input: x = -121
// Output: false
// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
//
// Example 3:
//
// Input: x = 10
// Output: false
// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
//
//
//
// Constraints:
//
// -231 <= x <= 231 - 1
//
//
// Follow up: Could you solve it without converting the integer to a string?


pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let borrow = x.to_string();
    let x = borrow.split("").collect::<Vec<&str>>();
    let mut i = 0;
    let mut j = x.len()-1;
    while(i<j) {
        if x[i] != x[j] {
            return false;
        }
        i+=1;
        j-=1;
    }
    true
}


#[cfg(test)]

#[test]

fn test_is_palindrome() {
    let x = 121;
    assert_eq!(is_palindrome(x), true);

    let x = -121;
    assert_eq!(is_palindrome(x), false);

    let x = 10;
    assert_eq!(is_palindrome(x), false);
}