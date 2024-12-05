/*You are given an n x n integer matrix. You can do the following operation any number of times:

Choose any two adjacent elements of matrix and multiply each of them by -1.

Two elements are considered adjacent if and only if they share a border.

Your goal is to maximize the summation of the matrix's elements. Return the maximum sum of the matrix's elements using the operation mentioned above.



Example 1:

Input: matrix = [[1,-1],[-1,1]]
Output: 4
Explanation: We can follow the following steps to reach sum equals 4:
- Multiply the 2 elements in the first row by -1.
- Multiply the 2 elements in the first column by -1.

Example 2:

Input: matrix = [[1,2,3],[-1,-2,-3],[1,2,3]]
Output: 16
Explanation: We can follow the following step to reach sum equals 16:
- Multiply the 2 last elements in the second row by -1.



Constraints:

n == matrix.length == matrix[i].length
2 <= n <= 250
-105 <= matrix[i][j] <= 105

*/
pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
    let mut sum = 0i64;
    let mut min = i64::MAX;
    let mut negative_count = 0;

    for row in matrix {
        for val in row {
            sum += val.abs() as i64;
            if val < 0 {
                negative_count += 1;
            }
            min = (val.abs() as i64).min(min);
        }
    }

    sum + {
        if negative_count == 0 {
            0
        } else {
            -2 * min
        }
    }
}