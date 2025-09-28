use std::collections::HashMap;
pub fn min_arrivals_to_discard(arrivals: Vec<i32>, w: i32, m: i32) -> i32 {
    let w = w as usize;
    let mut map = HashMap::new();

    let mut result = 0;
    let n = arrivals.len();

    let mut left = 0;
    for right in 0..n {
        *map.entry(arrivals[right]).or_insert(0) += 1;

        if let Some(&count) = map.get(&arrivals[right]) {
            if count > m {
                for _ in left..right {
                    if let Some(count) = map.get_mut(&arrivals[left]) {
                        *count -= 1;
                    }
                    left += 1;
                }
                result += 1;
            }
        }

        if right - left + 1 == w {
            if let Some(count) = map.get_mut(&arrivals[left]) {
                *count -= 1;
            }
            left += 1;
        }
    }

    result
}
