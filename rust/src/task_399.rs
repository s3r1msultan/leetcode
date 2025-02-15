/*You are given an array of variable pairs equations and an array of real numbers values, where equations[i] = [Ai, Bi] and values[i] represent the equation Ai / Bi = values[i]. Each Ai or Bi is a string that represents a single variable.

You are also given some queries, where queries[j] = [Cj, Dj] represents the jth query where you must find the answer for Cj / Dj = ?.

Return the answers to all queries. If a single answer cannot be determined, return -1.0.

Note: The input is always valid. You may assume that evaluating the queries will not result in division by zero and that there is no contradiction.

Note: The variables that do not occur in the list of equations are undefined, so the answer cannot be determined for them.



Example 1:

Input: equations = [["a","b"],["b","c"]], values = [2.0,3.0], queries = [["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]]
Output: [6.00000,0.50000,-1.00000,1.00000,-1.00000]
Explanation:
Given: a / b = 2.0, b / c = 3.0
queries are: a / c = ?, b / a = ?, a / e = ?, a / a = ?, x / x = ?
return: [6.0, 0.5, -1.0, 1.0, -1.0 ]
note: x is undefined => -1.0

Example 2:

Input: equations = [["a","b"],["b","c"],["bc","cd"]], values = [1.5,2.5,5.0], queries = [["a","c"],["c","b"],["bc","cd"],["cd","bc"]]
Output: [3.75000,0.40000,5.00000,0.20000]

Example 3:

Input: equations = [["a","b"]], values = [0.5], queries = [["a","b"],["b","a"],["a","c"],["x","y"]]
Output: [0.50000,2.00000,-1.00000,-1.00000]



Constraints:

1 <= equations.length <= 20
equations[i].length == 2
1 <= Ai.length, Bi.length <= 5
values.length == equations.length
0.0 < values[i] <= 20.0
1 <= queries.length <= 20
queries[i].length == 2
1 <= Cj.length, Dj.length <= 5
Ai, Bi, Cj, Dj consist of lower case English letters and digits.

*/
use std::collections::HashMap;
#[derive(Debug)]
struct UnionFind {
    representatives: HashMap<String, String>,
    coefficients: HashMap<String, f64>,
}
impl UnionFind {
    fn new() -> Self {
        let mut representatives = HashMap::new();
        let mut coefficients = HashMap::new();
        Self {
            representatives,
            coefficients,
        }
    }

    fn find(&mut self, x: &String) -> (String, f64) {
        if self.representatives.contains_key(x) {
            let x_coefficient = self.coefficients.get(x).unwrap().clone();
            let parent = self.representatives.get(x).unwrap().clone();
            if &parent == x {
                return (x.clone(), x_coefficient.clone());
            }
            let (root, root_coefficient) = self.find(&parent);

            self.coefficients.insert(x.clone(), root_coefficient * x_coefficient);
            self.representatives.insert(x.clone(), root.clone());

            (root, x_coefficient * root_coefficient)
        } else {
            self.representatives.insert(x.clone(), x.clone());
            self.coefficients.insert(x.clone(), 1.0);
            (x.clone(), 1.0)
        }
    }

    fn union(&mut self, a: &String, b: &String, coefficient: f64) {
        let (root_a, coefficient_a) = self.find(a);
        let (root_b, coefficient_b) = self.find(b);


        if root_a == root_b {
            return;
        }
        self.representatives.insert(root_b.clone(), root_a.clone());
        self.coefficients.insert(root_b, coefficient * coefficient_a / coefficient_b);
    }

    fn calculate(&mut self, a: &String, b: &String) -> Option<f64> {
        if !self.representatives.contains_key(a) || !self.representatives.contains_key(b) {
            return None;
        }
        let (root_a, coefficient_a) = self.find(a);
        let (root_b, coefficient_b) = self.find(b);

        if root_a != root_b {
            return None;
        }

        Some(coefficient_a / coefficient_b)
    }
}

pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
    let n = equations.len();
    let mut result = vec![];
    let mut union_find = UnionFind::new();
    for i in 0..n {
        let a = &equations[i][0];
        let b = &equations[i][1];
        let coefficient = values[i];

        union_find.union(a, b, coefficient);
    }

    for query in &queries {
        let a = &query[0];
        let b = &query[1];
        if let Some(x) = union_find.calculate(a, b) {
            result.push(x);
        } else {
            result.push(-1.0);
        }
    }

    result
}