pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
    let n = n as usize;

    let mut result = 1;
    let mut count = 0;

    let delay = delay as usize;
    let forget = forget as usize;

    let mut prefix_sum = vec![0; n + 1];
    prefix_sum[delay] += 1;
    prefix_sum[forget + 1] -= 1;

    for i in 0..n {
        if i + delay <= n {
            prefix_sum[i + delay] += count;
        }
        if i + forget + 1 <= n {
            prefix_sum[i + forget + 1] -= count;
        }
        result += count;
        count += prefix_sum[i];
    }

    result
}
