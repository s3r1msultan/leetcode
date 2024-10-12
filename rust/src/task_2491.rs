/*You are given a positive integer array skill of even length n where skill[i] denotes the skill of the ith player. Divide the players into n / 2 teams of size 2 such that the total skill of each team is equal.

The chemistry of a team is equal to the product of the skills of the players on that team.

Return the sum of the chemistry of all the teams, or return -1 if there is no way to divide the players into teams such that the total skill of each team is equal.



Example 1:

Input: skill = [3,2,5,1,3,4]
Output: 22
Explanation:
Divide the players into the following teams: (1, 5), (2, 4), (3, 3), where each team has a total skill of 6.
The sum of the chemistry of all the teams is: 1 * 5 + 2 * 4 + 3 * 3 = 5 + 8 + 9 = 22.

Example 2:

Input: skill = [3,4]
Output: 12
Explanation:
The two players form a team with a total skill of 7.
The chemistry of the team is 3 * 4 = 12.

Example 3:

Input: skill = [1,1,2,3]
Output: -1
Explanation:
There is no way to divide the players into teams such that the total skill of each team is equal.



Constraints:

2 <= skill.length <= 105
skill.length is even.
1 <= skill[i] <= 1000

*/

fn divide_players(skill: Vec<i32>) -> i64 {
	let mut sum = 0;
	let mut map = std::collections::HashMap::new();

	for &val in &skill {
		sum += val;
		*map.entry(val).or_insert(0) += 1;
	}

	let n = skill.len();
	let mut product = 0;
	let count_of_groups = n as i32 / 2;
	let sum_of_group = sum / count_of_groups;
	if sum_of_group % 2 == 0 {
		if let Some(count) = map.get(&(sum_of_group / 2)) {
			if count % 2 == 1 {
				return -1;
			}
		}
	}
	// println!("{sum}");
	// println!("{count_of_groups}");
	// println!("{sum_of_group}");
	// println!("{map:#?}");

	for &val in &skill {
		if let Some(&first_count) = map.get(&val) {
			if first_count == 0 {
				continue;
			}
			if let Some(&second_count) = map.get(&(sum_of_group - val)) {
				if second_count != 0 {
					product += val as i64 * (sum_of_group - val) as i64;
					*map.entry(val).or_insert(0) -= 1;
					*map.entry(sum_of_group - val).or_insert(0) -= 1;
				} else {
					return -1;
				}
			} else {
				return -1;
			}
		}
	}


	product
}