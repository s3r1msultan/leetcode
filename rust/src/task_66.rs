// You are given a large integer represented as an integer array digits, where each digits[i] is the ith digit of the integer. The digits are ordered from most significant to least significant in left-to-right order. The large integer does not contain any leading 0's.
//
// Increment the large integer by one and return the resulting array of digits.
//
//
//
// Example 1:
//
// Input: digits = [1,2,3]
// Output: [1,2,4]
// Explanation: The array represents the integer 123.
// Incrementing by one gives 123 + 1 = 124.
// Thus, the result should be [1,2,4].
//
// Example 2:
//
// Input: digits = [4,3,2,1]
// Output: [4,3,2,2]
// Explanation: The array represents the integer 4321.
// Incrementing by one gives 4321 + 1 = 4322.
// Thus, the result should be [4,3,2,2].
//
// Example 3:
//
// Input: digits = [9]
// Output: [1,0]
// Explanation: The array represents the integer 9.
// Incrementing by one gives 9 + 1 = 10.
// Thus, the result should be [1,0].
//
//
//
// Constraints:
//
// 1 <= digits.length <= 100
// 0 <= digits[i] <= 9
// digits does not contain any leading 0's.
//
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    let len = digits.len();

    let mut i = len - 1;
    let mut res = if digits[i] + 1 == 10 {
        1
    } else {
        digits[i]+=1;
        0
    };

    while res != 0 {
        digits[i]+=1;
        res = 0;
        if digits[i] == 10 {
            digits[i] = 0;
            res = 1;
        }
        if i == 0 {
            break;
        }
        i-=1;
    }

    if res == 1 {
        [vec![1], digits].concat()
    } else {
        digits
    }
}

#[cfg(test)]

#[test]

fn test_plus_one() {
    let digits = vec![1,2,3];
    assert_eq!(plus_one(digits), vec![1,2,4]);

    let digits = vec![4,3,2,1];
    assert_eq!(plus_one(digits), vec![4,3,2,2]);

    let digits = vec![9];
    assert_eq!(plus_one(digits), vec![1, 0]);
}