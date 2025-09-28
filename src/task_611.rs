use std::collections::hash_map::Values;

/* Given an integer array nums, return the number of triplets chosen from the array that can make triangles if we take them as side lengths of a triangle.



Example 1:

Input: nums = [2,2,3,4]
Output: 3
Explanation: Valid combinations are:
2,3,4 (using the first 2)
2,3,4 (using the second 2)
2,2,3

Example 2:

Input: nums = [4,2,3,4]
Output: 4



Constraints:

    1 <= nums.length <= 1000
    0 <= nums[i] <= 1000

  */
pub fn triangle_number(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut result = 0;

    let mut frequency_map = HashMap::new();

    for &num in &nums {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    let mut values = frequency_map
        .iter()
        .map(|(_, &val)| val)
        .collect::<Vec<_>>();

    let n = values.len();

    for i in 0..n {
        let a = values[i];
        *frequency_map.entry(a).or_insert(0) -= 1;

        for j in i + 1..n {
            let c = values[i];

            if let Some(count) = frequency_map.get_mut(&c) {
                if *count <= 1 {
                    continue;
                }
                *count -= 1;
            }

            *frequency_map.entry(key)
        }

        *frequency_map.entry(a).or_insert(0) += 1;
    }

    result
}
