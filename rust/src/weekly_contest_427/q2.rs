/*You are given an array points where points[i] = [xi, yi] represents the coordinates of a point on an infinite plane.

Your task is to find the maximum area of a rectangle that:

Can be formed using four of these points as its corners.
Does not contain any other point inside or on its border.
Has its edges parallel to the axes.

Return the maximum area that you can obtain or -1 if no such rectangle is possible.



Example 1:

Input: points = [[1,1],[1,3],[3,1],[3,3]]

Output: 4

Explanation:

Example 1 diagram

We can make a rectangle with these 4 points as corners and there is no other point that lies inside or on the border. Hence, the maximum possible area would be 4.

Example 2:

Input: points = [[1,1],[1,3],[3,1],[3,3],[2,2]]

Output: -1

Explanation:

Example 2 diagram

There is only one rectangle possible is with points [1,1], [1,3], [3,1] and [3,3] but [2,2] will always lie inside it. Hence, returning -1.

Example 3:

Input: points = [[1,1],[1,3],[3,1],[3,3],[1,2],[3,2]]

Output: 2

Explanation:

Example 3 diagram

The maximum area rectangle is formed by the points [1,3], [1,2], [3,2], [3,3], which has an area of 2. Additionally, the points [1,1], [1,2], [3,1], [3,2] also form a valid rectangle with the same area.



Constraints:

1 <= points.length <= 10
points[i].length == 2
0 <= xi, yi <= 100
All the given points are unique.

*/

pub fn max_rectangle_area(points: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashSet;
    let points_set: HashSet<(i32, i32)> = points.iter().map(|a| (a[0], a[1])).collect();
    let mut max_area = -1;

    for i in 0..points.len() {
        let (x1, y1) = (points[i][0], points[i][1]);

        for j in i + 1..points.len() {
            let (x2, y2) = (points[j][0], points[j][1]);

            if x1 != x2 && y1 != y2 {
                let corner_1 = (x1, y2);
                let corner_2 = (x2, y1);

                if points_set.contains(&corner_1) && points_set.contains(&corner_2) {
                    let area = (x2 - x1).abs() * (y2 - y1).abs();

                    let mut is_valid = true;

                    for &(x, y) in points_set.iter() {
                        if (x == x1 && y == y1) || (x == x2 && y == y2) || (x == x1 && y == y2) || (x == x2 && y == y1) {
                            continue;
                        }
                        if x <= x1.max(x2) && x >= x1.min(x2) && y <= y1.max(y2) && y >= y1.min(y2) {
                            is_valid = false;
                            break;
                        }
                    }

                    if is_valid {
                        max_area = max_area.max(area);
                    }
                }
            }
        }
    }

    max_area
}