pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }
        gcd(b, a % b)
    }
    fn lcm(a: i32, b: i32) -> i32 {
        ((a as i64 * b as i64) / gcd(a, b) as i64) as i32
    }

    let mut stack = vec![];

    for mut num in nums {
        while let Some(&peek) = stack.last() {
            if gcd(peek, num) > 1 {
                num = lcm(num, peek);
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(num);
    }
    stack
}
