/*Given an integer array arr, return true if there are three consecutive odd numbers in the array. Otherwise, return false.



Example 1:

Input: arr = [2,6,4,1]
Output: false
Explanation: There are no three consecutive odds.

Example 2:

Input: arr = [1,2,34,3,4,5,7,23,12]
Output: true
Explanation: [5,7,23] are three consecutive odds.



Constraints:

1 <= arr.length <= 1000
1 <= arr[i] <= 1000
*/

fn three_consecutive_odds(arr: Vec<i32>) -> bool {
	if arr.len() < 3 {
		return false;
	}
	fn is_odd(n: i32) -> bool {
		n % 2 == 1
	}
	for i in 0..(arr.len() - 2) {
		if is_odd(arr[i]) && is_odd(arr[i + 1]) && is_odd(arr[i + 2]) {
			return true;
		}
	}
	false
}


#[cfg(test)]
#[test]
fn test_three_consecutive_odds() {
	let arr = vec![2, 6, 4, 1];
	let result = false;
	assert_eq!(three_consecutive_odds(arr), result);
	let arr = vec![1, 2, 34, 3, 4, 5, 7, 23, 12];
	let result = true;
	assert_eq!(three_consecutive_odds(arr), result);
}