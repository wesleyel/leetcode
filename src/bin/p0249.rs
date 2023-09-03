/**
 * [249] Group Shifted Strings
 *
 * We can shift a string by shifting each of its letters to its successive letter.
 *
 *
 * 	For example, "abc" can be shifted to be "bcd".
 *
 *
 * We can keep shifting the string to form a sequence.
 *
 *
 * 	For example, we can keep shifting "abc" to form the sequence: "abc" -> "bcd" -> ... -> "xyz".
 *
 *
 * Given an array of strings strings, group all strings[i] that belong to the same shifting sequence. You may return the answer in any order.
 *
 *
 * Example 1:
 * Input: strings = ["abc","bcd","acef","xyz","az","ba","a","z"]
 * Output: [["acef"],["a","z"],["abc","bcd","xyz"],["az","ba"]]
 * Example 2:
 * Input: strings = ["a"]
 * Output: [["a"]]
 *
 *
 * Constraints:
 *
 *
 * 	1 <= strings.length <= 200
 * 	1 <= strings[i].length <= 50
 * 	strings[i] consists of lowercase English letters.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/group-shifted-strings/
// discussion: https://leetcode.cn/problems/group-shifted-strings/discussion/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;
impl Solution {
    pub fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for i in strings.iter() {
            let key = Solution::string_hash(i);
            let val = map.get(&key).unwrap_or(&vec![]).to_vec();
            map.insert(key, [val, vec![i.to_string()]].concat());
        }
        map.values().map(|x| x.to_vec()).collect()
    }

    pub fn string_hash(s: &String) -> String {
        let fisrt = s.as_bytes()[0] as i32;
        let res = String::new();
        s.chars().into_iter().fold(res, |mut acc, x| {
            acc.push_str(&format!("{}-", (x as i32 - fisrt + 26) % 26));
            acc
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    macro_rules! vec_string {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }
    use super::*;

    #[test]
    fn test_249() {
        assert_eq!(
            Solution::group_strings(vec_string![
                "abc", "bcd", "acef", "xyz", "az", "ba", "a", "z"
            ])
            .sort(),
            vec![
                vec_string!["abc", "bcd", "xyz"],
                vec_string!["az", "ba"],
                vec_string!["acef"],
                vec_string!["a", "z"]
            ]
            .sort()
        );
        assert_eq!(
            Solution::group_strings(vec_string!["a"]).sort(),
            vec![vec_string!["a"]].sort()
        );
    }
}
