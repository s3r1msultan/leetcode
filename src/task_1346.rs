/*Given an array arr of integers, check if there exist two indices i and j such that :

i != j
0 <= i, j < arr.length
arr[i] == 2 * arr[j]



Example 1:

Input: arr = [10,2,5,3]
Output: true
Explanation: For i = 0 and j = 2, arr[i] == 10 == 2 * 5 == 2 * arr[j]

Example 2:

Input: arr = [3,1,7,11]
Output: false
Explanation: There is no i and j that satisfy the conditions.



Constraints:

2 <= arr.length <= 500
-103 <= arr[i] <= 103

*/

pub fn check_if_exist(arr: Vec<i32>) -> bool {
    use std::collections::HashMap;

    let mut map = HashMap::new();

    for &val in &arr {
        *map.entry(val).or_insert(0) += 1;
    }

    for val in arr {
        if let Some(&count) = map.get(&(val * 2)) {
            if count > 2 {
                return true;
            }
        }
    }

    false
}