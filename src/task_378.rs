/*Given an n x n matrix where each of the rows and columns is sorted in ascending order, return the kth smallest element in the matrix.

Note that it is the kth smallest element in the sorted order, not the kth distinct element.

You must find a solution with a memory complexity better than O(n2).



Example 1:

Input: matrix = [[1,5,9],[10,11,13],[12,13,15]], k = 8
Output: 13
Explanation: The elements in the matrix are [1,5,9,10,11,12,13,13,15], and the 8th smallest number is 13

Example 2:

Input: matrix = [[-5]], k = 1
Output: -5



Constraints:

n == matrix.length == matrix[i].length
1 <= n <= 300
-109 <= matrix[i][j] <= 109
All the rows and columns of matrix are guaranteed to be sorted in non-decreasing order.
1 <= k <= n2
*/

/*pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut pq = BinaryHeap::new();

    for i in 0..n {
        for j in 0..m {
            pq.push(matrix[i][j]);
            if pq.len() > k as usize {
                pq.pop();
            }
        }
    }

    let mut kth_element = 0;
    for _ in 0..k {
        kth_element = pq.pop().unwrap();
    }
    kth_element
}*/


pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    fn count(mid: usize, matrix: &Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let m = matrix[0].len();
        let n = mid / m;
        let m = mid % m;
        for i in 0..n {}
        count
    }
    let n = matrix.len();
    let m = matrix[0].len();
    let mut left = 0;
    let mut right = n * m;

    while left < right {
        let mid = left + (right - left) / 2;
        if count(mid, &matrix) >= k {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    matrix[left / m][left % m]
}