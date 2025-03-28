/*You have information about n different recipes. You are given a string array recipes and a 2D string array ingredients. The ith recipe has the name recipes[i], and you can create it if you have all the needed ingredients from ingredients[i]. A recipe can also be an ingredient for other recipes, i.e., ingredients[i] may contain a string that is in recipes.

You are also given a string array supplies containing all the ingredients that you initially have, and you have an infinite supply of all of them.

Return a list of all the recipes that you can create. You may return the answer in any order.

Note that two recipes may contain each other in their ingredients.



Example 1:

Input: recipes = ["bread"], ingredients = [["yeast","flour"]], supplies = ["yeast","flour","corn"]
Output: ["bread"]
Explanation:
We can create "bread" since we have the ingredients "yeast" and "flour".

Example 2:

Input: recipes = ["bread","sandwich"], ingredients = [["yeast","flour"],["bread","meat"]], supplies = ["yeast","flour","meat"]
Output: ["bread","sandwich"]
Explanation:
We can create "bread" since we have the ingredients "yeast" and "flour".
We can create "sandwich" since we have the ingredient "meat" and can create the ingredient "bread".

Example 3:

Input: recipes = ["bread","sandwich","burger"], ingredients = [["yeast","flour"],["bread","meat"],["sandwich","meat","bread"]], supplies = ["yeast","flour","meat"]
Output: ["bread","sandwich","burger"]
Explanation:
We can create "bread" since we have the ingredients "yeast" and "flour".
We can create "sandwich" since we have the ingredient "meat" and can create the ingredient "bread".
We can create "burger" since we have the ingredient "meat" and can create the ingredients "bread" and "sandwich".



Constraints:

n == recipes.length == ingredients.length
1 <= n <= 100
1 <= ingredients[i].length, supplies.length <= 100
1 <= recipes[i].length, ingredients[i][j].length, supplies[k].length <= 10
recipes[i], ingredients[i][j], and supplies[k] consist only of lowercase English letters.
All the values of recipes and supplies combined are unique.
Each ingredients[i] does not contain any duplicate values.*/

pub fn find_all_recipes(recipes: Vec<String>, ingredients: Vec<Vec<String>>, supplies: Vec<String>) -> Vec<String> {
    use std::collections::{HashMap, HashSet};
    let n = recipes.len();
    let mut is_visited = HashSet::new();
    let mut is_reachable = HashSet::new();

    for supply in supplies {
        is_reachable.insert(supply);
    }

    let mut recipes_map = HashMap::new();

    for i in 0..n {
        let name = recipes[i].clone();
        let ingredients = ingredients[i].clone();
        recipes_map.insert(name, ingredients);
    }

    let mut result = vec![];

    for i in 0..n {
        if can_be_cooked(&recipes[i], &recipes_map, &mut is_reachable, &mut is_visited) {
            result.push(recipes[i].clone());
            continue;
        }
    }

    fn can_be_cooked(recipe: &String, recipes_map: &HashMap<String, Vec<String>>, is_reachable: &mut HashSet<String>, is_visited: &mut HashSet<String>) -> bool {
        if is_visited.contains(recipe) {
            return is_reachable.contains(recipe);
        }
        is_visited.insert(recipe.clone());

        if let Some(ingredients) = recipes_map.get(recipe) {
            let mut ingredient_count = 0;

            for ingredient in ingredients {
                if is_reachable.contains(ingredient) || can_be_cooked(ingredient, recipes_map, is_reachable, is_visited) {
                    ingredient_count += 1;
                }
            }

            if ingredients.len() == ingredient_count {
                is_reachable.insert(recipe.clone());
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    result
}