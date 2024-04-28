// Given a rows x cols binary matrix filled with 0's and 1's, find the largest rectangle containing only 1's and return its area.
//
//
//
// Example 1:
//
// Input: matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
// Output: 6
// Explanation: The maximal rectangle is shown in the above picture.
//
// Example 2:
//
// Input: matrix = [["0"]]
// Output: 0
//
// Example 3:
//
// Input: matrix = [["1"]]
// Output: 1
//
//
//
// Constraints:
//
// rows == matrix.length
// cols == matrix[i].length
// 1 <= row, cols <= 200
// matrix[i][j] is '0' or '1'.
//



pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }

    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut heights = vec![0; cols];
    let mut max_area = 0;

    for i in 0..rows {
        for j in 0..cols {
            if matrix[i][j] == '1' {
                heights[j] += 1;
            } else {
                heights[j] = 0;
            }
        }

        max_area = max_area.max(largest_rectangle_area(&heights))
    }

    max_area
}

fn largest_rectangle_area(heights: &Vec<i32>) -> i32 {
    let mut stack: Vec<usize> = Vec::new();
    let mut max_area = 0;
    let mut extended_heights = heights.clone();
    extended_heights.push(0);

    for i in 0..extended_heights.len() {
        while let Some(&index) = stack.last() {
            if extended_heights[index] > extended_heights[i] {
                let h = extended_heights[stack.pop().unwrap()];
                let width = if stack.is_empty() {
                    i as i32
                } else {
                    i as i32 - *stack.last().unwrap() as i32 - 1
                };

                max_area = max_area.max(h*width);
            } else {
                break
            }
        }
        stack.push(i);
    }

    max_area
}


#[cfg(test)]

#[test]

fn test_maximal_rectangle() {
    let matrix: Vec<Vec<char>> = vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0']
    ];

    assert_eq!(maximal_rectangle(matrix), 6);


    let matrix = vec![vec!['0']];
    assert_eq!(maximal_rectangle(matrix), 0);

    let matrix = vec![vec!['1']];
    assert_eq!(maximal_rectangle(matrix), 1);

}