/*You are given an array of strings names, and an array heights that consists of distinct positive integers. Both arrays are of length n.

For each index i, names[i] and heights[i] denote the name and height of the ith person.

Return names sorted in descending order by the people's heights.



Example 1:

Input: names = ["Mary","John","Emma"], heights = [180,165,170]
Output: ["Mary","Emma","John"]
Explanation: Mary is the tallest, followed by Emma and John.

Example 2:

Input: names = ["Alice","Bob","Bob"], heights = [155,185,150]
Output: ["Bob","Alice","Bob"]
Explanation: The first Bob is the tallest, followed by Alice and the second Bob.



Constraints:

n == names.length == heights.length
1 <= n <= 103
1 <= names[i].length <= 20
1 <= heights[i] <= 105
names[i] consists of lower and upper case English letters.
All the values of heights are distinct.

*/

fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
	let mut names = names;
	let mut zipped = names.into_iter().zip(heights.into_iter()).collect::<Vec<_>>();
	zipped.sort_unstable_by(|a, b| a.1.cmp(&b.1));
	names = zipped.iter().map(|zipped| zipped.0.clone()).collect();
	names
}