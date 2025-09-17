pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
    let n = mat.len();
    let m = mat[0].len();

    let mut row = vec![vec![0; m]; n];
    let mut result = 0;

    for i in 0..n {
        for j in 0..m {
            row[i][j] = if j == 0 {
                mat[i][j]
            } else {
                if mat[i][j] == 1 {
                    row[i][j - 1] + 1
                } else {
                    0
                }
            };

            let mut count = row[i][j];

            for i in (0..=i).rev() {
                count = count.min(row[i][j]);

                if count == 0 {
                    break;
                }

                result += count;
            }
        }
    }

    result
}
