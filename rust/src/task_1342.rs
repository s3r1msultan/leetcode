// Given an integer num, return the number of steps to reduce it to zero.
//
// In one step, if the current number is even, you have to divide it by 2, otherwise, you have to subtract 1 from it.
//
//
//
// Example 1:
//
// Input: num = 14
// Output: 6
// Explanation:
// Step 1) 14 is even; divide by 2 and obtain 7.
// Step 2) 7 is odd; subtract 1 and obtain 6.
// Step 3) 6 is even; divide by 2 and obtain 3.
// Step 4) 3 is odd; subtract 1 and obtain 2.
// Step 5) 2 is even; divide by 2 and obtain 1.
// Step 6) 1 is odd; subtract 1 and obtain 0.
//
// Example 2:
//
// Input: num = 8
// Output: 4
// Explanation:
// Step 1) 8 is even; divide by 2 and obtain 4.
// Step 2) 4 is even; divide by 2 and obtain 2.
// Step 3) 2 is even; divide by 2 and obtain 1.
// Step 4) 1 is odd; subtract 1 and obtain 0.
//
// Example 3:
//
// Input: num = 123
// Output: 12

// pub fn number_of_steps(num: i32) -> i32 {
// 	let mut num = num;
// 	let mut steps = 0;
// 	while num != 0 {
// 		num>>1;
// 		steps+=1;
// 	}
// 	steps
// }

pub fn number_of_steps(num: i32) -> i32 {
	if num > 1 {
		(num.count_zeros() - num.leading_zeros() + 2 * num.count_ones() - 1) as i32
	} else {
		num
	}
}
#[cfg(test)]
#[test]

fn test_number_of_steps() {
	let num = 14;
	let result = 6;
	assert_eq!(number_of_steps(num), result);

	let num = 8;
	let result = 4;
	assert_eq!(number_of_steps(num), result);
	let num = 123;
	let result = 12;
	assert_eq!(number_of_steps(num), result);
}