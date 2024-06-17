/*An integer divisible by the sum of its digits is said to be a Harshad number. You are given an integer x. Return the sum of the digits of x if x is a Harshad number, otherwise, return -1.



Example 1:

Input: x = 18

Output: 9

Explanation:

The sum of digits of x is 9. 18 is divisible by 9. So 18 is a Harshad number and the answer is 9.

Example 2:

Input: x = 23

Output: -1

Explanation:

The sum of digits of x is 5. 23 is not divisible by 5. So 23 is not a Harshad number and the answer is -1.



Constraints:

1 <= x <= 100

*/

fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
	let mut sum = 0;
	let mut cloned = x.clone();
	while cloned != 0 {
		sum+=cloned%10;
		cloned/=10;
	}
	if x % sum == 0 {
		return sum;
	}
	-1
}


#[cfg(test)]
#[test]

fn test_sum_of_the_digits_of_harshad_number() {
	let x = 18;
	let result = 9;
	assert_eq!(sum_of_the_digits_of_harshad_number(x), result);

	let x = 23;
	let result = -1;
	assert_eq!(sum_of_the_digits_of_harshad_number(x), result);


}