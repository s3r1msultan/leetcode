/*Given a string s which consists of lowercase or uppercase letters, return the length of the longest
palindrome
that can be built with those letters.

Letters are case sensitive, for example, "Aa" is not considered a palindrome.



Example 1:

Input: s = "abccccdd"
Output: 7
Explanation: One longest palindrome that can be built is "dccaccd", whose length is 7.

Example 2:

Input: s = "a"
Output: 1
Explanation: The longest palindrome that can be built is "a", whose length is 1.



Constraints:

1 <= s.length <= 2000
s consists of lowercase and/or uppercase English letters only.

*/

use std::collections::HashMap;
fn longest_palindrome(s: String) -> i32 {
	let mut map: HashMap<char, i32> = HashMap::new();
	for char in s.chars() {
		map.entry(char).and_modify(|x| *x+=1).or_insert(1);
	}
	let mut result = 0;
	let mut max_odd = 0;
	for &num in map.values() {
		if num%2==0 {
			result+=num;
		} else {
			if max_odd < num {
				result+=(max_odd-1).max(0);
				max_odd=num;
			} else {
				result+=num-1;
			}
		}
	}

	result+max_odd
}

#[cfg(test)]

#[test]

fn test_longest_palindrome() {
	let s = "abccccdd".to_string();
	let result = 7;
	assert_eq!(longest_palindrome(s), result);

	let s = "a".to_string();
	let result = 1;
	assert_eq!(longest_palindrome(s), result);

	let s = "civilwartestingwhetherthatnaptionoranynartionsoconceivedandsodedicatedcanlongendureWeareqmetonagreatbattlefiemldoftzhatwarWehavecometodedicpateaportionofthatfieldasafinalrestingplaceforthosewhoheregavetheirlivesthatthatnationmightliveItisaltogetherfangandproperthatweshoulddothisButinalargersensewecannotdedicatewecannotconsecratewecannothallowthisgroundThebravelmenlivinganddeadwhostruggledherehaveconsecrateditfaraboveourpoorponwertoaddordetractTgheworldadswfilllittlenotlenorlongrememberwhatwesayherebutitcanneverforgetwhattheydidhereItisforusthelivingrathertobededicatedheretotheulnfinishedworkwhichtheywhofoughtherehavethusfarsonoblyadvancedItisratherforustobeherededicatedtothegreattdafskremainingbeforeusthatfromthesehonoreddeadwetakeincreaseddevotiontothatcauseforwhichtheygavethelastpfullmeasureofdevotionthatweherehighlyresolvethatthesedeadshallnothavediedinvainthatthisnationunsderGodshallhaveanewbirthoffreedomandthatgovernmentofthepeoplebythepeopleforthepeopleshallnotperishfromtheearth".to_string();
	let result = 983;
	assert_eq!(longest_palindrome(s), result);
}