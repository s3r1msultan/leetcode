/*Given an array nums sorted in non-decreasing order, return the maximum between the number of positive integers and the number of negative integers.

In other words, if the number of positive integers in nums is pos and the number of negative integers is neg, then return the maximum of pos and neg.

Note that 0 is neither positive nor negative.



Example 1:

Input: nums = [-2,-1,-1,1,2,3]
Output: 3
Explanation: There are 3 positive integers and 3 negative integers. The maximum count among them is 3.

Example 2:

Input: nums = [-3,-2,-1,0,0,1,2]
Output: 3
Explanation: There are 2 positive integers and 3 negative integers. The maximum count among them is 3.

Example 3:

Input: nums = [5,20,66,1314]
Output: 4
Explanation: There are 4 positive integers and 0 negative integers. The maximum count among them is 4.



Constraints:

1 <= nums.length <= 2000
-2000 <= nums[i] <= 2000
nums is sorted in a non-decreasing order.



Follow up: Can you solve the problem in O(log(n)) time complexity?
*/

/*pub fn maximum_count(nums: Vec<i32>) -> i32 {
    let mut positive_count = 0;
    let mut negative_count = 0;
    for num in nums {
        if num > 0 {
            positive_count += 1;
        }
        if num < 0 {
            negative_count += 1;
        }
    }
    positive_count.max(negative_count)
}*/

pub fn maximum_count(nums: Vec<i32>) -> i32 {
    fn binary_search(start: usize, end: usize, arr: &Vec<i32>) -> i32 {
        if start > end {
            return 0;
        }

        if start == end {
            return if arr[start] < 0 {
                start as i32
            } else if arr[start] > 0 {
                (arr.len() - start) as i32
            } else {
                0
            };
        }



        let mid = start + (end - start) / 2;
        if arr[mid] == 0 {
           binary_search(start, mid-1, arr).max(binary_search(mid+1, end, arr))
        } else if arr[mid] < 0 {
            (mid as i32).max(binary_search(mid+1, end, arr))
        } else {
            ((arr.len() - mid) as i32).max(binary_search(start, mid, arr))
        }
    }

    binary_search(0, nums.len() - 1, &nums)
}