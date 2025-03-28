/*A good meal is a meal that contains exactly two different food items with a sum of deliciousness equal to a power of two.

You can pick any two different foods to make a good meal.

Given an array of integers deliciousness where deliciousness[i] is the deliciousness of the i​​​​​​th​​​​​​​​ item of food, return the number of different good meals you can make from this list modulo 109 + 7.

Note that items with different indices are considered different even if they have the same deliciousness value.



Example 1:

Input: deliciousness = [1,3,5,7,9]
Output: 4
Explanation: The good meals are (1,3), (1,7), (3,5) and, (7,9).
Their respective sums are 4, 8, 8, and 16, all of which are powers of 2.

Example 2:

Input: deliciousness = [1,1,1,3,3,3,7]
Output: 15
Explanation: The good meals are (1,1) with 3 ways, (1,3) with 9 ways, and (1,7) with 3 ways.



Constraints:

1 <= deliciousness.length <= 105
0 <= deliciousness[i] <= 220

*/

pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut map = std::collections::HashMap::new();
    let mut set = std::collections::HashSet::new();

    for num in deliciousness {
        *map.entry(num).or_insert(0) += 1;
    }

    let mut count = 0i64;

    for (&num_1, &count_1) in map.iter() {
        for i in 0..21 {
            let num_2 = (1 << i) - num_1;
            if set.contains(&num_2) {
                continue;
            }
            if num_1 == num_2 {
                count += count_1;
            } else if let Some(&count_2) = map.get(&num_2) {
                count += count_1 * count_2;
            }
            count %= MOD;
        }
        set.insert(num_1);
    }

    count as i32
}