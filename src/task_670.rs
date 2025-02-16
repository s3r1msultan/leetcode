/*You are given an integer num. You can swap two digits at most once to get the maximum valued number.

Return the maximum valued number you can get.



Example 1:

Input: num = 2736
Output: 7236
Explanation: Swap the number 2 and the number 7.

Example 2:

Input: num = 9973
Output: 9973
Explanation: No swap.



Constraints:

0 <= num <= 108

*/

fn maximum_swap(num: i32) -> i32 {
	let mut num = num.to_string().chars().collect::<Vec<_>>();
	let mut max = num.clone();
	let n = num.len();

	for i in 0..n {
		for j in i..n {
			num.swap(i, j);
			max = max.max(num.clone());
			num.swap(i, j);
		}
	}


	max.iter().collect::<String>().parse::<i32>().unwrap()
}