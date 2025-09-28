pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
    use std::collections::HashSet;
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }
        gcd(b, a % b)
    }

    let gcd = gcd(numerator, denominator);
    let mut numerator = numerator / gcd;
    let denominator = denominator / gcd;

    let mut result = (numerator / denominator).to_string();

    numerator %= denominator;

    if numerator == 0 {
        return result;
    }

    result.push('.');

    let mut set = HashSet::new();

    let mut s = "".to_string();
    let init_numerator = numerator;
    while numerator != 0 {
        while numerator < denominator {
            s.push('0');
            numerator *= 10;
        }
        s.pop();

        numerator %= denominator;

        if numerator == init_numerator {
            break;
        }
    }

    if numerator == init_numerator {
        result.push('(');
        result.push_str(&s);
        result.push(')');
    }

    result
}
