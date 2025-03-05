pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let mut left_arr = vec![];
    let mut right_arr = vec![];
    let n = nums.len();
    for i in 0..n {
        if nums[i] < pivot {
            left_arr.push(nums[i]);
        } else {
            right_arr.push(nums[i]);
        }
    }
    vec![left_arr, right_arr].iter().flatten().collect()
}