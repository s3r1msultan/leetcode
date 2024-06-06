/*Alice has some number of cards and she wants to rearrange the cards into groups so that each group is of size groupSize, and consists of groupSize consecutive cards.

Given an integer array hand where hand[i] is the value written on the ith card and an integer groupSize, return true if she can rearrange the cards, or false otherwise.



Example 1:

Input: hand = [1,2,3,6,2,3,4,7,8], groupSize = 3
Output: true
Explanation: Alice's hand can be rearranged as [1,2,3],[2,3,4],[6,7,8]

Example 2:

Input: hand = [1,2,3,4,5], groupSize = 4
Output: false
Explanation: Alice's hand can not be rearranged into groups of 4.



Constraints:

1 <= hand.length <= 104
0 <= hand[i] <= 109
1 <= groupSize <= hand.length

 */

use std::collections::BTreeMap;

fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
	if hand.len() % group_size as usize != 0 {
		return false;
	}

	let mut map: BTreeMap<i32, i32> = BTreeMap::new();
	for i in hand {
		*map.entry(i).or_insert(0) += 1;
	}


	while let Some((&card, &count)) = map.iter().next() {
		println!("{card} {count}");
		for i in 0..group_size {
			if let Some(new_count) = map.get_mut(&(card+i)) {
				if *new_count < count {
					return false;
				}
				*new_count-=count;
				if *new_count ==0 {
					map.remove(&(card + i));
				}
			} else {
				return false;
			}

		}
	}

	true
}

#[cfg(test)]

#[test]

fn test_is_n_straight_hand() {
	let hand = vec![1,2,3,6,2,3,4,7,8];
	let group_size = 3;
	let result = true;
	assert_eq!(is_n_straight_hand(hand, group_size), result);

	let hand = vec![1,2,3,4,5];
	let group_size = 4;
	let result = false;
	assert_eq!(is_n_straight_hand(hand, group_size), result);
}