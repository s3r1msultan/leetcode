/*We are given an array asteroids of integers representing asteroids in a row.

For each asteroid, the absolute value represents its size, and the sign represents its direction (positive meaning right, negative meaning left). Each asteroid moves at the same speed.

Find out the state of the asteroids after all collisions. If two asteroids meet, the smaller one will explode. If both are the same size, both will explode. Two asteroids moving in the same direction will never meet.



Example 1:

Input: asteroids = [5,10,-5]
Output: [5,10]
Explanation: The 10 and -5 collide resulting in 10. The 5 and 10 never collide.

Example 2:

Input: asteroids = [8,-8]
Output: []
Explanation: The 8 and -8 collide exploding each other.

Example 3:

Input: asteroids = [10,2,-5]
Output: [10]
Explanation: The 2 and -5 collide resulting in -5. The 10 and -5 collide resulting in 10.



Constraints:

2 <= asteroids.length <= 104
-1000 <= asteroids[i] <= 1000
asteroids[i] != 0

*/


pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
	use std::cmp::Ordering;
	
	let mut stack: Vec<i32> = vec![];

	for asteroid in asteroids {
		let mut asteroid = asteroid;
		while let Some(&last) = stack.last() {
			if last > 0 && asteroid < 0 {
				match last.cmp(&(-asteroid)) {
					Ordering::Less => {
						stack.pop();
					}
					Ordering::Equal => {
						stack.pop();
						asteroid = 0;
						break;
					}
					Ordering::Greater => {
						asteroid = 0;
						break;
					}
				}
			} else {
				break;
			}
		}
		if asteroid != 0 {
			stack.push(asteroid);
		}
	}

	stack
}

#[cfg(test)]
#[test]
fn test_asteroid_collision() {
	let asteroids = vec![5, 10, -5];
	let result = vec![5, 10];
	assert_eq!(asteroid_collision(asteroids), result);


	let asteroids = vec![8, -8];
	let result = vec![];
	assert_eq!(asteroid_collision(asteroids), result);


	let asteroids = vec![10, 2, -5];
	let result = vec![10];
	assert_eq!(asteroid_collision(asteroids), result);
}