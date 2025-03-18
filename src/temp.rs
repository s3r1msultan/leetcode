pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
    let mut candies = candies;
    candies.sort_unstable();
    if candies.iter().fold(0, |acc, &curr| acc + curr as i64) == k {
        return 0;
    }
    let n = candies.len();
    let k = k as usize;
    let mut start = 0;
    let mut end = n - 1;

    while start < end {
        let mid = start + (end - start) / 2;
        if n - mid >= k {
            start = mid + 1;
        } else {
            end = mid;
        }
    }

    candies[start]
}