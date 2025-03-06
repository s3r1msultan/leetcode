pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let n = grid.len();
    let mut result = vec![0, 0];
    let mut set = vec![false; n*n];
    for i in 0..n {
        for j in 0..n {
            if set[i * n + j - 1] {
                result[0] = grid[i][j];
            } else {
                set[i * n + j - 1] = true;
            }
        }
    }
    for i in 0..n {
        for j in 0..n {
            if !set[i * n + j - 1] {
                result[1] = grid[i][j];
                return result;
            }
        }
    }
    result
}