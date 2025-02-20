/*You have n  tiles, where each tile has one letter tiles[i] printed on it.

Return the number of possible non-empty sequences of letters you can make using the letters printed on those tiles.



Example 1:

Input: tiles = "AAB"
Output: 8
Explanation: The possible sequences are "A", "B", "AA", "AB", "BA", "AAB", "ABA", "BAA".

Example 2:

Input: tiles = "AAABBC"
Output: 188

Example 3:

Input: tiles = "V"
Output: 1



Constraints:

1 <= tiles.length <= 7
tiles consists of uppercase English letters.

*/

pub fn num_tile_possibilities(tiles: String) -> i32 {
    fn backtrack(curr_tile: &mut String, result: &mut HashSet<String>, used: &mut Vec<bool>, tiles: &Vec<char>) {
        result.insert(curr_tile.clone());

        if curr_tile.len() == tiles.len() {
            return;
        }

        for i in 0..tiles.len() {
            if used[i] {
                continue;
            }

            used[i] = true;

            curr_tile.push(tiles[i]);

            backtrack(curr_tile, result, used, tiles);

            curr_tile.pop();

            used[i] = false;
        }

    }
    use std::collections::HashSet;
    let tiles = tiles.chars().collect::<Vec<char>>();
    let n = tiles.len();
    let mut used = vec![false; n];
    let mut curr_tile = String::new();
    let mut result = HashSet::new();

    backtrack(&mut curr_tile, &mut result, &mut used, &tiles);

    result.len() as i32
}
