/*There are n children standing in a line. Each child is assigned a rating value given in the integer array ratings.

You are giving candies to these children subjected to the following requirements:

Each child must have at least one candy.
Children with a higher rating get more candies than their neighbors.

Return the minimum number of candies you need to have to distribute the candies to the children.



Example 1:

Input: ratings = [1,0,2]
Output: 5
Explanation: You can allocate to the first, second and third child with 2, 1, 2 candies respectively.

Example 2:

Input: ratings = [1,2,2]
Output: 4
Explanation: You can allocate to the first, second and third child with 1, 2, 1 candies respectively.
The third child gets 1 candy because it satisfies the above two conditions.



Constraints:

n == ratings.length
1 <= n <= 2 * 104
0 <= ratings[i] <= 2 * 104

*/

pub fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    let mut result = vec![1; n];

    for i in 1..n-1 {
        let mut max = 0;
        if ratings[i] > ratings[i + 1] {
            max = max.max(result[i + 1]);
        }
        if ratings[i - 1] < ratings[i] {
            max = max.max(result[i - 1]);
        }
        result[i] = max + 1;
    }

    for i in (1..n-1).rev() {
        let mut max = 0;
        if ratings[i] > ratings[i + 1] {
            max = max.max(result[i + 1]);
        }
        if ratings[i - 1] < ratings[i] {
            max = max.max(result[i - 1]);
        }
        result[i] = max + 1;
    }

    if n > 1 {
        result[0] += if ratings[0] > ratings[1] {
            result[1]
        } else {
            0
        };
        result[n - 1] += if ratings[n-1] > ratings[n-2] {
            result[n - 2]
        } else {
            0
        };
    }

    result.into_iter().fold(0, |acc, curr| acc + curr)
}