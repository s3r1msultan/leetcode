/*Given an unsorted integer array nums. Return the smallest positive integer that is not present in nums.

You must implement an algorithm that runs in O(n) time and uses O(1) auxiliary space.



Example 1:

Input: nums = [1,2,0]
Output: 3
Explanation: The numbers in the range [1,2] are all in the array.

Example 2:

Input: nums = [3,4,-1,1]
Output: 2
Explanation: 1 is in the array but 2 is missing.

Example 3:

Input: nums = [7,8,9,11,12]
Output: 1
Explanation: The smallest positive integer 1 is missing.



Constraints:

1 <= nums.length <= 105
-231 <= nums[i] <= 231 - 1

*/

/*pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let mut min = 1;
    for num in nums {
        if min == num {
            min += 1;
        } else if num > min {
            return min;
        }
    }

    min
}*/

/*pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut set = std::collections::HashSet::new();

    let mut min = 1;

    for num in nums {
        set.insert(num);
    }

    while set.contains(&min) {
        min += 1;
    }

    min
}*/

pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut nums = nums;

    let mut has_1 = false;
    for i in 0..n {
        if nums[i] == 1 {
            has_1 = true;
        }

        if nums[i] <= 0 || nums[i] > n as i32 {
            nums[i] = 1;
        }
    }

    if !has_1 {
        return 1;
    }

    for i in 0..n {
        let value = nums[i].abs() as usize;
        if value == n {
            nums[0] = -nums[i].abs();
        } else {
            nums[value] = -nums[value].abs();
        }
    }

    for i in 1..n {
        if nums[i] > 0 {
            return i as i32;
        }
    }

    if nums[0] > 0 {
        return n as i32;
    }

    n as i32 + 1
}