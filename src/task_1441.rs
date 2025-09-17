pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
    let mut result = vec![];
    let mut i = 0;
    let mut count = 1;
    while i < target.len() {
        while count < target[i] && count < n {
            result.push("Push".to_string());
            result.push("Pop".to_string());
            count += 1;
        }
        result.push("Push".to_string());
        count += 1;
        i += 1;
    }

    result
}
