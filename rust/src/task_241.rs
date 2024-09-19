/*Given a string expression of numbers and operators, return all possible results from computing all the different possible ways to group numbers and operators. You may return the answer in any order.

The test cases are generated such that the output values fit in a 32-bit integer and the number of different results does not exceed 104.



Example 1:

Input: expression = "2-1-1"
Output: [0,2]
Explanation:
((2-1)-1) = 0
(2-(1-1)) = 2

Example 2:

Input: expression = "2*3-4*5"
Output: [-34,-14,-10,-10,10]
Explanation:
(2*(3-(4*5))) = -34
((2*3)-(4*5)) = -14
((2*(3-4))*5) = -10
(2*((3-4)*5)) = -10
(((2*3)-4)*5) = 10



Constraints:

1 <= expression.length <= 20
expression consists of digits and the operator '+', '-', and '*'.
All the integer values in the input expression are in the range [0, 99].
The integer values in the input expression do not have a leading '-' or '+' denoting the sign.

*/


// Recursion solution
// fn diff_ways_to_compute(expression: String) -> Vec<i32> {
// 	if expression.len() <= 2 {
// 		return vec![expression.parse::<i32>().unwrap()];
// 	}
// 	let mut result = vec![];
// 	let chars = expression.split("").collect::<Vec<_>>();
// 	for i in 0..chars.len() {
// 		match chars[i] {
// 			"*" | "+" | "-" => {
// 				let first_part = diff_ways_to_compute(chars[..i].join(""));
// 				let second_part = diff_ways_to_compute(chars[i + 1..].join(""));
// 				for &num1 in first_part.iter() {
// 					for &num2 in second_part.iter() {
// 						match chars[i] {
// 							"+" => result.push(num1 + num2),
// 							"-" => result.push(num1 - num2),
// 							"*" => result.push(num1 * num2),
// 							_ => {}
// 						}
// 					}
// 				}
// 			}
// 			_ => {}
// 		}
// 	}
// 	result
// }


// Memoization

fn diff_ways_to_compute(expression: String) -> Vec<i32> {
	let n = expression.len();
	let mut memo: Vec<Vec<Vec<i32>>> = vec![vec![vec![]; n]; n];
	fn helper(start: usize, end: usize, expression: &str, memo: &mut Vec<Vec<Vec<i32>>>) -> Vec<i32> {
		if !memo[start][end].is_empty() {
			return memo[start][end].clone();
		}

		if start == end || (start == end - 1 && expression.chars().nth(start).unwrap().is_digit(10)) {
			return vec![expression[start..=end].parse::<i32>().unwrap()];
		}

		let mut result = vec![];
		for i in start..=end {
			let char = expression.chars().nth(i).unwrap();
			if char.is_digit(10) {
				continue;
			}
			let first_part = helper(start, i - 1, expression, memo);
			let second_part = helper(i + 1, end, expression, memo);
			for &num1 in first_part.iter() {
				for &num2 in second_part.iter() {
					match char {
						'+' => result.push(num1 + num2),
						'-' => result.push(num1 - num2),
						'*' => result.push(num1 * num2),
						_ => {}
					}
				}
			}
		}

		result
	}
	helper(0, n - 1, &expression, &mut memo)
}