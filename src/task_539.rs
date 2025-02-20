/*Given a list of 24-hour clock time points in "HH:MM" format, return the minimum minutes difference between any two time-points in the list.



Example 1:

Input: timePoints = ["23:59","00:00"]
Output: 1

Example 2:

Input: timePoints = ["00:00","23:59","00:00"]
Output: 0



Constraints:

2 <= timePoints.length <= 2 * 104
timePoints[i] is in the format "HH:MM".
*/


// Using Sorting:
// Time complexity: O(n log n)
// Space complexity: O(n)
//
// fn find_min_difference(time_points: Vec<String>) -> i32 {
// 	let n = time_points.len();
// 	let mut minutes = time_points.iter().map(|s| s.split(':').collect::<Vec<_>>())
// 		.collect::<Vec<_>>().iter()
// 		.map(|s| s[0].parse::<i32>().unwrap() * 60 + s[1].parse::<i32>().unwrap()).collect::<Vec<i32>>();
//
// 	minutes.sort_unstable_by(|a, b| a.cmp(&b));
// 	let mut min_diff = i32::MAX;
// 	println!("{minutes:?}");
// 	for i in 1..n {
// 		min_diff = min_diff.min(minutes[i] - minutes[i - 1]);
// 	}
// 	min_diff = min_diff.min(minutes[0] + 1440 - minutes[n - 1]);
// 	min_diff
// }

// Using Bucket sort
// Time complexity: O(n)
// Space complexity: O(1)
fn find_min_difference(time_points: Vec<String>) -> i32 {
	let n = time_points.len();
	if n > 1440 {
		return 0;
	}
	let mut minutes = [false; 1440];
	for time_point in time_points.iter() {
		let hours = time_point[0..2].parse::<i32>().unwrap();
		let mins = time_point[3..].parse::<i32>().unwrap();
		let overall_minutes = (hours * 60 + mins) as usize;
		if minutes[overall_minutes] {
			return 0;
		}
		minutes[overall_minutes] = true;
	}

	let mut prev_index = usize::MAX;
	let mut first_index = usize::MAX;
	let mut min_diff = usize::MAX;

	for i in 0..1440 {
		if minutes[i] {
			if prev_index != usize::MAX {
				min_diff = min_diff.min(i - prev_index);
			}
			prev_index = i;
			if first_index == usize::MAX {
				first_index = i;
			}
		}
	}

	min_diff.min(first_index + 1440 - prev_index) as i32
}