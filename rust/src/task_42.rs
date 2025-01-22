// Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.
//
//
//
// Example 1:
//
// Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
// Output: 6
// Explanation: The above elevation map (black section) is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped.
//
// Example 2:
//
// Input: height = [4,2,0,3,2,5]
// Output: 9
//
//
//
// Constraints:
//
// n == height.length
// 1 <= n <= 2 * 104
// 0 <= height[i] <= 105
//

pub fn trap(height: Vec<i32>) -> i32 {
    let mut stack = vec![];
    let mut res = 0;
    for (index, &val) in height.iter().enumerate() {
        while !stack.is_empty() && val > height[*stack.last().unwrap()] {
            let popped_index = stack.pop().unwrap();
            if stack.is_empty() {
                break;
            }

            let distance = index - stack.last().unwrap() - 1;
            let min_height = val.min(height[stack.last().unwrap()]) - height[popped_index];
            res += min_height * distance as i32;
        }
        stack.push(index)
    }
    res
}

#[cfg(test)]
#[test]

fn test_trap() {
    let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    assert_eq!(trap(height), 6);

    let height = vec![4, 2, 0, 3, 2, 5];
    assert_eq!(trap(height), 9)
}