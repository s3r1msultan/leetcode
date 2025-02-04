/*You are given an integer array nums containing positive integers. We define a function encrypt such that encrypt(x) replaces every digit in x with the largest digit in x. For example, encrypt(523) = 555 and encrypt(213) = 333.

Return the sum of encrypted elements.



Example 1:

Input: nums = [1,2,3]

Output: 6

Explanation: The encrypted elements are [1,2,3]. The sum of encrypted elements is 1 + 2 + 3 == 6.

Example 2:

Input: nums = [10,21,31]

Output: 66

Explanation: The encrypted elements are [11,22,33]. The sum of encrypted elements is 11 + 22 + 33 == 66.



Constraints:

1 <= nums.length <= 50
1 <= nums[i] <= 1000

*/

pub fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
    let mut sum = 0;

    for num in nums {
        let mut temp = num;
        let mut n = 0;
        let mut max = 0;
        while temp != 0 {
            max = max.max(temp % 10);
            temp /= 10;
            n += 1;
        }
        let mut num = 0;
        while n > 0 {
            num *= 10;
            num += max;
            n -= 1;
        }
        sum += num;
    }

    sum
}