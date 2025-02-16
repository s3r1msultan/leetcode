/*There are n 1-indexed robots, each having a position on a line, health, and movement direction.

You are given 0-indexed integer arrays positions, healths, and a string directions (directions[i] is either 'L' for left or 'R' for right). All integers in positions are unique.

All robots start moving on the line simultaneously at the same speed in their given directions. If two robots ever share the same position while moving, they will collide.

If two robots collide, the robot with lower health is removed from the line, and the health of the other robot decreases by one. The surviving robot continues in the same direction it was going. If both robots have the same health, they are both removed from the line.

Your task is to determine the health of the robots that survive the collisions, in the same order that the robots were given, i.e. final heath of robot 1 (if survived), final health of robot 2 (if survived), and so on. If there are no survivors, return an empty array.

Return an array containing the health of the remaining robots (in the order they were given in the input), after no further collisions can occur.

Note: The positions may be unsorted.




Example 1:

Input: positions = [5,4,3,2,1], healths = [2,17,9,15,10], directions = "RRRRR"
Output: [2,17,9,15,10]
Explanation: No collision occurs in this example, since all robots are moving in the same direction. So, the health of the robots in order from the first robot is returned, [2, 17, 9, 15, 10].

Example 2:

Input: positions = [3,5,2,6], healths = [10,10,15,12], directions = "RLRL"
Output: [14]
Explanation: There are 2 collisions in this example. Firstly, robot 1 and robot 2 will collide, and since both have the same health, they will be removed from the line. Next, robot 3 and robot 4 will collide and since robot 4's health is smaller, it gets removed, and robot 3's health becomes 15 - 1 = 14. Only robot 3 remains, so we return [14].

Example 3:

Input: positions = [1,2,5,6], healths = [10,10,11,11], directions = "RLRL"
Output: []
Explanation: Robot 1 and robot 2 will collide and since both have the same health, they are both removed. Robot 3 and 4 will collide and since both have the same health, they are both removed. So, we return an empty array, [].



Constraints:

1 <= positions.length == healths.length == directions.length == n <= 105
1 <= positions[i], healths[i] <= 109
directions[i] == 'L' or directions[i] == 'R'
All values in positions are distinct

*/


fn survived_robots_healths(positions: Vec<i32>, healths: Vec<i32>, directions: String) -> Vec<i32> {
	use std::cmp::Ordering;

	#[derive(Debug, Clone, Copy)]
	struct Robot {
		position: i32,
		health: i32,
		direction: char,
		index: usize,
	}

	let mut robots: Vec<Robot> = positions.iter().zip(healths.iter().zip(directions.chars().collect::<Vec<char>>().iter())).enumerate().map(|(index, (&position, (&health, &direction)))| Robot { position, health, direction, index }).collect();
	robots.sort_unstable_by(|a, b| a.position.cmp(&b.position));
	let mut stack: Vec<Robot> = vec![];

	for robot in robots {
		let mut current_robot = robot;
		let mut equal = false;
		while let Some(mut last) = stack.pop() {
			if last.direction == 'R' && current_robot.direction == 'L' {
				match current_robot.health.cmp(&last.health) {
					Ordering::Less => {
						last.health -= 1;
						stack.push(last);
						break;
					}
					Ordering::Equal => {
						equal = true;
						break;
					}
					Ordering::Greater => {
						current_robot.health -= 1;
					}
				}
			} else {
				stack.push(last);
				stack.push(current_robot);
				break;
			}
		}
		if stack.is_empty() && !equal {
			stack.push(current_robot);
		}
	}

	stack.sort_by(|a, b| a.index.cmp(&b.index));
	stack.iter().map(|robot| robot.health).collect::<Vec<i32>>()
}

#[cfg(test)]
#[test]
fn test_survived_robots_healths() {
	let positions = vec![3, 5, 2, 6];
	let healths = vec![10, 10, 11, 11];
	let directions = "RLRL".to_string();
	let result = [];
	assert_eq!(survived_robots_healths(positions, healths, directions), result);

	let positions = vec![3, 5, 2, 6];
	let healths = vec![10, 10, 15, 12];
	let directions = "RLRL".to_string();
	let result = [14];
	assert_eq!(survived_robots_healths(positions, healths, directions), result);

	let positions = vec![5, 4, 3, 2, 1];
	let healths = vec![2, 17, 9, 15, 10];
	let directions = "RRRRR".to_string();
	let result = [2, 17, 9, 15, 10];
	assert_eq!(survived_robots_healths(positions, healths, directions), result);
}