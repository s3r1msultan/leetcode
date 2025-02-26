/*You are given an array of positive integers nums.

An array arr is called product equivalent if prod(arr) == lcm(arr) * gcd(arr), where:

prod(arr) is the product of all elements of arr.
gcd(arr) is the
GCD
of all elements of arr.
lcm(arr) is the
LCM
of all elements of arr.

Return the length of the longest product equivalent
subarray
of nums.



Example 1:

Input: nums = [1,2,1,2,1,1,1]

Output: 5

Explanation:

The longest product equivalent subarray is [1, 2, 1, 1, 1], where prod([1, 2, 1, 1, 1]) = 2, gcd([1, 2, 1, 1, 1]) = 1, and lcm([1, 2, 1, 1, 1]) = 2.

Example 2:

Input: nums = [2,3,4,5,6]

Output: 3

Explanation:

The longest product equivalent subarray is [3, 4, 5].

Example 3:

Input: nums = [1,2,3,1,4,5,1]

Output: 5



Constraints:

2 <= nums.length <= 100
1 <= nums[i] <= 10

*/

pub fn max_length(nums: Vec<i32>) -> i32 {
    fn gcd(a: i32, b:i32) -> i32 {
        if a % b == 0 {
            return b;
        }

        gcd(b, a % b)
    }
    fn lcm(a: i32, b: i32) -> i32 {
        (a / gcd(a,b)) * b
    }

    let n = nums.len();

    let mut left = 0;

    let mut max_length = 0;

    let mut curr_gcd = 0;
    let mut curr_lcm = 1;
    let mut curr_product = 1;

    for right in 1..n {
        curr_product *= nums[right];

        while curr_product != curr_gcd * curr_lcm && left < right {
            curr_product /= nums[left];

            left += 1;
        }

        curr_gcd = gcd(curr_gcd, nums[right]);
        curr_lcm = lcm(curr_lcm, nums[right]);
        max_length = max_length.max(right - left + 1);

    }

    max_length as i32
}