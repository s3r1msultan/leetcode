<<<<<<< HEAD
/*You are given a 0-indexed integer array nums of length n.

You can perform the following operation as many times as you want:

Pick an index i that you haven’t picked before, and pick a prime p strictly less than nums[i], then subtract p from nums[i].

Return true if you can make nums a strictly increasing array using the above operation and false otherwise.

A strictly increasing array is an array whose each element is strictly greater than its preceding element.



Example 1:

Input: nums = [4,9,6,10]
Output: true
Explanation: In the first operation: Pick i = 0 and p = 3, and then subtract 3 from nums[0], so that nums becomes [1,9,6,10].
In the second operation: i = 1, p = 7, subtract 7 from nums[1], so nums becomes equal to [1,2,6,10].
After the second operation, nums is sorted in strictly increasing order, so the answer is true.

Example 2:

Input: nums = [6,8,11,12]
Output: true
Explanation: Initially nums is sorted in strictly increasing order, so we don't need to make any operations.

Example 3:

Input: nums = [5,8,3]
Output: false
Explanation: It can be proven that there is no way to perform operations to make nums sorted in strictly increasing order, so the answer is false.



Constraints:

1 <= nums.length <= 1000
1 <= nums[i] <= 1000
nums.length == n

*/

pub fn prime_sub_operation(nums: Vec<i32>) -> bool {
    let max_element = *nums.iter().max().unwrap();

    let mut prime_numbers = vec![true; (max_element + 1) as usize];
    prime_numbers[0] = false;
    prime_numbers[1] = false;

    for i in 2..=((max_element as f64).sqrt() as usize) {
        if prime_numbers[i] {
            for j in (i * i..=max_element as usize).step_by(i) {
                prime_numbers[j] = false;
            }
        }
    }

    let mut curr_value = 1;
    let mut i = 0;

    while i < nums.len() {
        let difference = nums[i] - curr_value;

        if difference < 0 {
            return false;
        }

        if difference == 0 || prime_numbers[difference as usize] {
            i += 1;
        }

        curr_value += 1;
    }

    true
=======
/*

You are given a 0-indexed integer array nums of length n.

You can perform the following operation as many times as you want:

Pick an index i that you haven’t picked before, and pick a prime p strictly less than nums[i], then subtract p from nums[i].

Return true if you can make nums a strictly increasing array using the above operation and false otherwise.

A strictly increasing array is an array whose each element is strictly greater than its preceding element.



Example 1:

Input: nums = [4,9,6,10]
Output: true
Explanation: In the first operation: Pick i = 0 and p = 3, and then subtract 3 from nums[0], so that nums becomes [1,9,6,10].
In the second operation: i = 1, p = 7, subtract 7 from nums[1], so nums becomes equal to [1,2,6,10].
After the second operation, nums is sorted in strictly increasing order, so the answer is true.

Example 2:

Input: nums = [6,8,11,12]
Output: true
Explanation: Initially nums is sorted in strictly increasing order, so we don't need to make any operations.

Example 3:

Input: nums = [5,8,3]
Output: false
Explanation: It can be proven that there is no way to perform operations to make nums sorted in strictly increasing order, so the answer is false.



Constraints:

1 <= nums.length <= 1000
1 <= nums[i] <= 1000
nums.length == n

*/

pub fn prime_sub_operation(nums: Vec<i32>) -> bool {
    fn create_prime_numbers(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut prime_numbers = vec![true; n +1];
        let mut p: usize = 2;
        while p * p <= n {
            if prime_numbers[p] {
                for i in p*p..=n {
                    if i % p == 0 {
                        prime_numbers[i] = false;
                    }
                }
            }
            p += 1
        }
        prime_numbers.into_iter().enumerate().filter(|a| a.1).map(|a| a.0 as i32).collect()
    }
    let max = nums.iter().max().unwrap().clone();
    let prime_numbers = create_prime_numbers(max);
    println!("{:?}", prime_numbers);
    true
}

#[cfg(test)]
#[test]
fn test_prime_numbers() {
    let nums = vec![4,9,6,10];
    prime_sub_operation(nums);
>>>>>>> 2276d2c53b5e5f30758ccd90b506909ec90665d7
}