pub struct Solution {}

impl Solution {}

fn parse_vec(s: &str) -> Vec<i32> {
    s.trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect()
}
