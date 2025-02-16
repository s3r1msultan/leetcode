/*Given an integer array nums, return the maximum result of nums[i] XOR nums[j], where 0 <= i <= j < n.



Example 1:

Input: nums = [3,10,5,25,2,8]
Output: 28
Explanation: The maximum result is 5 XOR 25 = 28.

Example 2:

Input: nums = [14,70,53,83,49,91,36,80,92,51,66,70]
Output: 127



Constraints:

1 <= nums.length <= 2 * 105
0 <= nums[i] <= 231 - 1

*/
#[derive(Debug)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 2],
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: Default::default(),
        }
    }
}

#[derive(Debug)]
struct Trie {
    root: Box<TrieNode>,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: Box::new(TrieNode::new())
        }
    }

    fn insert(&mut self, x: i32) {
        let mut node = &mut self.root;

        for i in (0..32).rev() {
            let digit = (x >> i & 1) as usize;
            if node.children[digit].is_none() {
                node.children[digit] = Some(Box::new(TrieNode::new()));
            }
            node = node.children[digit].as_mut().unwrap();
        }
    }

    fn max_xor(&self, x: i32) -> i32 {
        let mut node = &self.root;
        let mut max_xor = 0;
        for i in (0..32).rev() {
            let mut digit = 1 ^ (x >> i & 1) as usize;
            if node.children[digit].is_none() {
                digit ^= 1;
            }
            max_xor |= (digit << i) as i32;
            node = node.children[digit].as_ref().unwrap();
        }
        max_xor ^ x
    }
}

pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
    let mut trie = Trie::new();
    for &num in &nums {
        trie.insert(num);
    }
    let mut max_xor = 0;
    for &num in &nums {
        max_xor = max_xor.max(trie.max_xor(num));
    }
    max_xor
}