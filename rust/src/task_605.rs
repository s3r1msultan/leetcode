/*You have a long flowerbed in which some of the plots are planted, and some are not. However, flowers cannot be planted in adjacent plots.

Given an integer array flowerbed containing 0's and 1's, where 0 means empty and 1 means not empty, and an integer n, return true if n new flowers can be planted in the flowerbed without violating the no-adjacent-flowers rule and false otherwise.



Example 1:

Input: flowerbed = [1,0,0,0,1], n = 1
Output: true

Example 2:

Input: flowerbed = [1,0,0,0,1], n = 2
Output: false



Constraints:

1 <= flowerbed.length <= 2 * 104
flowerbed[i] is 0 or 1.
There are no two adjacent flowers in flowerbed.
0 <= n <= flowerbed.length

*/

fn can_place_flowers(flowered: Vec<i32>, n: i32) -> bool {
	let mut flowered = flowered;
	let mut count = 0;
	for i in 0..flowered.len() {
		if flowered[i] == 0 {
			let mut flag = true;
			if (i >= 1 && flowered[i - 1] == 1) {
				flag = false;
			}

			if (i + 1 < flowered.len() && flowered[i + 1] == 1) {
				flag = false;
			}

			if flag {
				flowered[i] = 1;
				count += 1;
			}
		}
	}
	count >= n
}