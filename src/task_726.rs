/*Given a string formula representing a chemical formula, return the count of each atom.

The atomic element always starts with an uppercase character, then zero or more lowercase letters, representing the name.

One or more digits representing that element's count may follow if the count is greater than 1. If the count is 1, no digits will follow.

For example, "H2O" and "H2O2" are possible, but "H1O2" is impossible.

Two formulas are concatenated together to produce another formula.

For example, "H2O2He3Mg4" is also a formula.

A formula placed in parentheses, and a count (optionally added) is also a formula.

For example, "(H2O2)" and "(H2O2)3" are formulas.

Return the count of all elements as a string in the following form: the first name (in sorted order), followed by its count (if that count is more than 1), followed by the second name (in sorted order), followed by its count (if that count is more than 1), and so on.

The test cases are generated so that all the values in the output fit in a 32-bit integer.



Example 1:

Input: formula = "H2O"
Output: "H2O"
Explanation: The count of elements are {'H': 2, 'O': 1}.

Example 2:

Input: formula = "Mg(OH)2"
Output: "H2MgO2"
Explanation: The count of elements are {'H': 2, 'Mg': 1, 'O': 2}.

Example 3:

Input: formula = "K4(ON(SO3)2)2"
Output: "K4N2O14S4"
Explanation: The count of elements are {'K': 4, 'N': 2, 'O': 14, 'S': 4}.



Constraints:

1 <= formula.length <= 1000
formula consists of English letters, digits, '(', and ')'.
formula is always valid.

*/

fn count_of_atoms(formula: String) -> String {
	use std::collections::HashMap;
	let mut stack: Vec<HashMap<String, i32>> = vec![HashMap::new()];
	let mut i = 0;
	let n = formula.len();
	let formula_chars: Vec<char> = formula.chars().collect();
	while i < n {
		if formula_chars[i].is_uppercase() {
			let mut element = formula_chars[i].to_string();
			i += 1;
			while i < n && formula_chars[i].is_lowercase() {
				element.push(formula_chars[i]);
				i += 1;
			}
			let mut count = 0;
			while i < n && formula_chars[i].is_digit(10) {
				count *= 10;
				count += (formula_chars[i] as u8 - '0' as u8) as i32;
				i += 1;
			}

			if count == 0 {
				count = 1;
			}

			*stack.last_mut().unwrap().entry(element).or_insert(0) += count;
		} else if formula_chars[i] == '(' {
			stack.push(HashMap::new());
			i += 1;
		} else if formula_chars[i] == ')' {
			i += 1;
			let mut multiplier = 0;
			while i < n && formula_chars[i].is_digit(10) {
				multiplier *= 10;
				multiplier += (formula_chars[i] as u8 - '0' as u8) as i32;
				i += 1;
			}
			if multiplier == 0 {
				multiplier = 1;
			}

			let last = stack.pop().unwrap();
			let prelast = stack.last_mut().unwrap();

			for (element, count) in last {
				*prelast.entry(element).or_insert(0) += count * multiplier;
			}
		}
	}

	let mut element_count: Vec<_> = stack.pop().unwrap().into_iter().collect();
	element_count.sort_unstable_by(|a, b| a.0.cmp(&b.0));

	let mut result = "".to_string();
	for (element, count) in element_count {
		result.push_str(&element);
		if count > 1 {
			result.push_str(&count.to_string());
		}
	}

	result
}

#[cfg(test)]
#[test]
fn test_count_of_atoms() {
	let formula = "H2O".to_string();
	let result = "H2O".to_string();
	assert_eq!(count_of_atoms(formula), result);

	let formula = "Mg(OH)2".to_string();
	let result = "H2MgO2".to_string();
	assert_eq!(count_of_atoms(formula), result);

	let formula = "K4(ON(SO3)2)2".to_string();
	let result = "K4N2O14S4".to_string();
	assert_eq!(count_of_atoms(formula), result);
}