// You are given an array people where people[i] is the weight of the ith person, and an infinite number of boats where each boat can carry a maximum weight of limit. Each boat carries at most two people at the same time, provided the sum of the weight of those people is at most limit.
//
// Return the minimum number of boats to carry every given person.
//
//
//
// Example 1:
//
// Input: people = [1,2], limit = 3
// Output: 1
// Explanation: 1 boat (1, 2)
//
// Example 2:
//
// Input: people = [3,2,2,1], limit = 3
// Output: 3
// Explanation: 3 boats (1, 2), (2) and (3)
//
// Example 3:
//
// Input: people = [3,5,3,4], limit = 5
// Output: 4
// Explanation: 4 boats (3), (3), (4), (5)
//
//
//
// Constraints:
//
// 1 <= people.length <= 5 * 104
// 1 <= people[i] <= limit <= 3 * 104
//

pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
	let mut people = people;
	people.sort_unstable();
	let mut boats = 0;
	let mut left = 0;
	let mut right = people.len();

	while left<right {
		right-=1;

		if people[left] + people[right] <= limit {
			left+=1;
		}

		boats+=1;
	}

	boats
}

#[cfg(test)]

#[test]

fn test_num_rescue_boats() {
	let people: Vec<i32> = vec![1,2];
	let limit = 3;
	assert_eq!(num_rescue_boats(people, limit), 1);

	let people: Vec<i32> = vec![3,2,2,1];
	let limit = 3;
	assert_eq!(num_rescue_boats(people, limit), 3);

	let people: Vec<i32> = vec![3,5,3,4];
	let limit = 5;
	assert_eq!(num_rescue_boats(people, limit), 4);
}