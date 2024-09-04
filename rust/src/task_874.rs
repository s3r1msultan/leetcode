/*A robot on an infinite XY-plane starts at point (0, 0) facing north. The robot can receive a sequence of these three possible types of commands:

-2: Turn left 90 degrees.
-1: Turn right 90 degrees.
1 <= k <= 9: Move forward k units, one unit at a time.

Some of the grid squares are obstacles. The ith obstacle is at grid point obstacles[i] = (xi, yi). If the robot runs into an obstacle, then it will instead stay in its current location and move on to the next command.

Return the maximum Euclidean distance that the robot ever gets from the origin squared (i.e. if the distance is 5, return 25).

Note:

North means +Y direction.
East means +X direction.
South means -Y direction.
West means -X direction.
There can be obstacle in [0,0].



Example 1:

Input: commands = [4,-1,3], obstacles = []
Output: 25
Explanation: The robot starts at (0, 0):
1. Move north 4 units to (0, 4).
2. Turn right.
3. Move east 3 units to (3, 4).
The furthest point the robot ever gets from the origin is (3, 4), which squared is 32 + 42 = 25 units away.

Example 2:

Input: commands = [4,-1,4,-2,4], obstacles = [[2,4]]
Output: 65
Explanation: The robot starts at (0, 0):
1. Move north 4 units to (0, 4).
2. Turn right.
3. Move east 1 unit and get blocked by the obstacle at (2, 4), robot is at (1, 4).
4. Turn left.
5. Move north 4 units to (1, 8).
The furthest point the robot ever gets from the origin is (1, 8), which squared is 12 + 82 = 65 units away.

Example 3:

Input: commands = [6,-1,-1,6], obstacles = []
Output: 36
Explanation: The robot starts at (0, 0):
1. Move north 6 units to (0, 6).
2. Turn right.
3. Turn right.
4. Move south 6 units to (0, 0).
The furthest point the robot ever gets from the origin is (0, 6), which squared is 62 = 36 units away.



Constraints:

1 <= commands.length <= 104
commands[i] is either -2, -1, or an integer in the range [1, 9].
0 <= obstacles.length <= 104
-3 * 104 <= xi, yi <= 3 * 104
The answer is guaranteed to be less than 231.
s
*/

pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
	use std::collections::HashSet;
	let obstacles_set: HashSet<(i32, i32)> = HashSet::from_iter(obstacles.into_iter().map(|v| (v[0], v[1])));

	let mut max_distance_sq = 0;
	let mut curr_x = 0;
	let mut curr_y = 0;
	let mut direction = 0;
	// 0 - up, 1 - right, 2 - down, 3 - left
	// north, east, south, west
	let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
	for command in commands {
		match command {
			-2 => {
				direction -= 1;
				direction %= 4;
			}
			-1 => {
				direction += 1;
				direction %= 4;
			}
			_ => {
				let (dx, dy) = directions[direction];
				for i in 0..command {
					let next_x = curr_x + dx;
					let next_y = curr_y + dy;
					if obstacles_set.contains(&(next_x, next_y)) {
						break;
					}

					curr_x = next_x;
					curr_y = next_y;
					max_distance_sq = max_distance_sq.max(curr_y * curr_y + curr_x * curr_x);
				}
			}
		}
	}
	max_distance_sq
}