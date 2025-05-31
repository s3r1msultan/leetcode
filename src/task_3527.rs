/*You are given a 2D string array responses where each responses[i] is an array of strings representing survey responses from the ith day.

Return the most common response across all days after removing duplicate responses within each responses[i]. If there is a tie, return the

response.



Example 1:

Input: responses = [["good","ok","good","ok"],["ok","bad","good","ok","ok"],["good"],["bad"]]

Output: "good"

Explanation:

After removing duplicates within each list, responses = [["good", "ok"], ["ok", "bad", "good"], ["good"], ["bad"]].
"good" appears 3 times, "ok" appears 2 times, and "bad" appears 2 times.
Return "good" because it has the highest frequency.

Example 2:

Input: responses = [["good","ok","good"],["ok","bad"],["bad","notsure"],["great","good"]]

Output: "bad"

Explanation:

After removing duplicates within each list we have responses = [["good", "ok"], ["ok", "bad"], ["bad", "notsure"], ["great", "good"]].
"bad", "good", and "ok" each occur 2 times.
The output is "bad" because it is the lexicographically smallest amongst the words with the highest frequency.



Constraints:

1 <= responses.length <= 1000
1 <= responses[i].length <= 1000
1 <= responses[i][j].length <= 10
responses[i][j] consists of only lowercase English letters
*/

pub fn find_common_response(responses: Vec<Vec<String>>) -> String {
    let mut map = std::collections::BTreeMap::new();

    for response in responses {
        let mut set = std::collections::BTreeSet::new();
        for s in response {
            set.insert(s);
        }

        for s in set {
            *map.entry(s).or_insert(0) += 1;
        }
    }

    let mut result = map.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0.clone();
    let mut count = map.values().max().unwrap().clone();

    for (key, value) in map {
        if value == count {
            result = result.min(key.clone());
            count = value;
        }
    }

    result
}