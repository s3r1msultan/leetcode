/*Given a list of non-negative integers nums, arrange them such that they form the largest number and return it.

Since the result may be very large, so you need to return a string instead of an integer.



Example 1:

Input: nums = [10,2]
Output: "210"

Example 2:

Input: nums = [3,30,34,5,9]
Output: "9534330"



Constraints:

1 <= nums.length <= 100
0 <= nums[i] <= 109

*/

fn largest_number(nums: Vec<i32>) -> String {
	let mut nums = nums.iter().map(|num| num.to_string()).collect::<Vec<_>>();
	// println!("{nums:?}");
	nums.sort_by(|a, b| {
		(b.to_owned() + a).cmp(&(a.to_owned() + b))
	});
	if &nums[0] == "0" {
		return "0".to_string();
	}
	// println!("{nums:?}");
	nums.join("")
}