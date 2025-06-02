/*Given an array of integers arr, return true if and only if it is a valid mountain array.

Recall that arr is a mountain array if and only if:

arr.length >= 3
There exists some i with 0 < i < arr.length - 1 such that:
arr[0] < arr[1] < ... < arr[i - 1] < arr[i]
arr[i] > arr[i + 1] > ... > arr[arr.length - 1]



Example 1:

Input: arr = [2,1]
Output: false

Example 2:

Input: arr = [3,5,5]
Output: false

Example 3:

Input: arr = [0,3,2,1]
Output: true



Constraints:

1 <= arr.length <= 104
0 <= arr[i] <= 104

*/

pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
    let mut is_decreasing = false;
    for i in 1..arr.len() {
        if arr[i] == arr[i - 1] {
            return false;
        }

        if arr[i - 1] > arr[i] {
            if !is_decreasing {
                is_decreasing = true;
            }
        } else {
            if is_decreasing {
                return false;
            }
        }
    }
    true
}