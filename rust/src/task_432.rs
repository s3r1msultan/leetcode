/*Design a data structure to store the strings' count with the ability to return the strings with minimum and maximum counts.

Implement the AllOne class:

AllOne() Initializes the object of the data structure.
inc(String key) Increments the count of the string key by 1. If key does not exist in the data structure, insert it with count 1.
dec(String key) Decrements the count of the string key by 1. If the count of key is 0 after the decrement, remove it from the data structure. It is guaranteed that key exists in the data structure before the decrement.
getMaxKey() Returns one of the keys with the maximal count. If no element exists, return an empty string "".
getMinKey() Returns one of the keys with the minimum count. If no element exists, return an empty string "".

Note that each function must run in O(1) average time complexity.



Example 1:

Input
["AllOne", "inc", "inc", "getMaxKey", "getMinKey", "inc", "getMaxKey", "getMinKey"]
[[], ["hello"], ["hello"], [], [], ["leet"], [], []]
Output
[null, null, null, "hello", "hello", null, "hello", "leet"]

Explanation
AllOne allOne = new AllOne();
allOne.inc("hello");
allOne.inc("hello");
allOne.getMaxKey(); // return "hello"
allOne.getMinKey(); // return "hello"
allOne.inc("leet");
allOne.getMaxKey(); // return "hello"
allOne.getMinKey(); // return "leet"



Constraints:

1 <= key.length <= 10
key consists of lowercase English letters.
It is guaranteed that for each call to dec, key is existing in the data structure.
At most 5 * 104 calls will be made to inc, dec, getMaxKey, and getMinKey.

*/
use std::collections::HashMap;

struct AllOne {
	d: HashMap<String, i32>,
	op: i32,
	res: String,
}

impl AllOne {
	fn new() -> Self {
		AllOne {
			d: HashMap::new(),
			op: 0,
			res: String::new(),
		}
	}

	fn inc(&mut self, key: String) {
		self.op = 0;
		*self.d.entry(key).or_insert(0) += 1;
	}

	fn dec(&mut self, key: String) {
		self.op = 0;
		if let Some(count) = self.d.get_mut(&key) {
			if *count > 1 {
				*count -= 1;
			} else {
				self.d.remove(&key);
			}
		}
	}

	fn get_max_key(&mut self) -> String {
		if self.op == 1 {
			return self.res.clone();
		}
		self.op = 1;
		if !self.d.is_empty() {
			let mut max_key = String::new();
			let mut max_value = i32::MIN;
			for (key, &value) in &self.d {
				if value > max_value {
					max_value = value;
					max_key = key.clone();
				}
			}
			self.res = max_key.clone();
			return max_key;
		}
		self.res.clear();
		String::new()
	}

	fn get_min_key(&mut self) -> String {
		if self.op == 2 {
			return self.res.clone();
		}
		self.op = 2;
		if !self.d.is_empty() {
			let mut min_key = String::new();
			let mut min_value = i32::MAX;
			for (key, &value) in &self.d {
				if value < min_value {
					min_value = value;
					min_key = key.clone();
				}
			}
			self.res = min_key.clone();
			return min_key;
		}
		self.res.clear();
		String::new()
	}
}

