/*There is a school that has classes of students and each class will be having a final exam. You are given a 2D integer array classes, where classes[i] = [passi, totali]. You know beforehand that in the ith class, there are totali total students, but only passi number of students will pass the exam.

You are also given an integer extraStudents. There are another extraStudents brilliant students that are guaranteed to pass the exam of any class they are assigned to. You want to assign each of the extraStudents students to a class in a way that maximizes the average pass ratio across all the classes.

The pass ratio of a class is equal to the number of students of the class that will pass the exam divided by the total number of students of the class. The average pass ratio is the sum of pass ratios of all the classes divided by the number of the classes.

Return the maximum possible average pass ratio after assigning the extraStudents students. Answers within 10-5 of the actual answer will be accepted.



Example 1:

Input: classes = [[1,2],[3,5],[2,2]], extraStudents = 2
Output: 0.78333
Explanation: You can assign the two extra students to the first class. The average pass ratio will be equal to (3/4 + 3/5 + 2/2) / 3 = 0.78333.

Example 2:

Input: classes = [[2,4],[3,9],[4,5],[2,10]], extraStudents = 4
Output: 0.53485



Constraints:

1 <= classes.length <= 105
classes[i].length == 2
1 <= passi <= totali <= 105
1 <= extraStudents <= 105

*/

pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;
    let mut max_average = 0f64;

    struct PassRatio {
        pass: i32,
        total: i32,
        gain: f64,
    }

    impl PassRatio {
        fn new(pass: i32, total: i32) -> Self {
            let gain = ((pass + 1) as f64 / (total + 1) as f64) - (pass as f64 / total as f64);
            PassRatio { pass, total, gain }
        }
        fn pass_ratio(&self) -> f64 {
            self.pass as f64 / self.total as f64
        }

        fn update(&mut self) {
            self.pass += 1;
            self.total += 1;
            self.gain = ((self.pass + 1) as f64 / (self.total + 1) as f64) - self.pass_ratio();
        }
    }

    impl Eq for PassRatio {}
    impl PartialEq<Self> for PassRatio {
        fn eq(&self, other: &Self) -> bool {
            self.pass_ratio().eq(&other.pass_ratio())
        }
    }
    impl PartialOrd<Self> for PassRatio {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.gain.partial_cmp(&other.gain)
        }
    }

    impl Ord for PassRatio {
        fn cmp(&self, other: &Self) -> Ordering {
            self.gain.partial_cmp(&other.gain).unwrap()
        }
    }

    let mut min_heap = BinaryHeap::new();
    for class in &classes {
        let pass = class[0];
        let total = class[1];
        min_heap.push(PassRatio::new(pass, total));
    }

    let mut extra_students = extra_students;
    while extra_students > 0 {
        let mut min_ratio = min_heap.pop().unwrap();
        min_ratio.update();
        min_heap.push(min_ratio);
        extra_students -= 1;
    }

    while let Some(min_ratio) = min_heap.pop() {
        max_average += min_ratio.pass_ratio();
    }

    max_average / classes.len() as f64
}