/*You are given two integers numBottles and numExchange.

numBottles represents the number of full water bottles that you initially have. In one operation, you can perform one of the following operations:

Drink any number of full water bottles turning them into empty bottles.
Exchange numExchange empty bottles with one full water bottle. Then, increase numExchange by one.

Note that you cannot exchange multiple batches of empty bottles for the same value of numExchange. For example, if numBottles == 3 and numExchange == 1, you cannot exchange 3 empty water bottles for 3 full bottles.

Return the maximum number of water bottles you can drink.



Example 1:

Input: numBottles = 13, numExchange = 6
Output: 15
Explanation: The table above shows the number of full water bottles, empty water bottles, the value of numExchange, and the number of bottles drunk.

Example 2:

Input: numBottles = 10, numExchange = 3
Output: 13
Explanation: The table above shows the number of full water bottles, empty water bottles, the value of numExchange, and the number of bottles drunk.



Constraints:

1 <= numBottles <= 100
1 <= numExchange <= 100

*/

pub fn max_bottles_drunk(num_bottles: i32, num_exchange: i32) -> i32 {
    let mut full_bottles = 0;
    let mut num_exchange = num_exchange;
    let mut empty_bottles = num_bottles;
    let mut drunk_bottles = num_bottles;
    while empty_bottles >= num_exchange {
        while empty_bottles >= num_exchange {
            full_bottles += 1;
            empty_bottles -= num_exchange;
            num_exchange += 1;
        }
        empty_bottles += full_bottles;
        drunk_bottles += full_bottles;
        full_bottles = 0;
    }
    drunk_bottles
}