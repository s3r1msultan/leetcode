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
        while b*b <= c {
            if a*a + b*b == c {
                return true;
            }
            b+=1;
        }
        a+=1;
        b=0;
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


}