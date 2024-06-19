/*Given a non-negative integer c, decide whether there're two integers a and b such that a2 + b2 = c.



Example 1:

Input: c = 5
Output: true
Explanation: 1 * 1 + 2 * 2 = 5

Example 2:

Input: c = 3
Output: false



Constraints:

0 <= c <= 231 - 1

*/


fn judge_square_sum(c: i32) -> bool {
    let mut a = 0;
    let mut b = 0;
    while a*a <= c {
        b = c - a*a;
        if binary_search(0, b, b) {
            return true;
        }
        a+=1;
    }
    fn binary_search(a: i32, b: i32, c: i32) -> bool {
        if a > b {
            return false;
        }

        let mid = a + (b-a)/2;
        return if mid * mid == c {
            true
        } else {
            binary_search(mid + 1, b, b)
        }
    }
    false
}

#[cfg(test)]
#[test]

fn test_judge_square_sum() {
    let c = 5;
    let result = true;
    assert_eq!(judge_square_sum(c), result);

    let c = 3;
    let result = false;
    assert_eq!(judge_square_sum(c), result);

    let c = 1000;
    let result = true;
    // assert_eq!(judge_square_sum(c),result);

}