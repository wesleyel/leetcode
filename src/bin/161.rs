// 161. 相隔为 1 的编辑距离

impl Solution {
    pub fn is_one_edit_distance(s: String, t: String) -> bool {
        let (len_s, len_t) = (s.len(), t.len());
        if len_s < len_t {
            return Solution::is_one_edit_distance(t, s);
        } else if len_s - len_t > 1 {
            return false;
        }

        let mut found = false;
        for (i, (x, y)) in s.bytes().zip(t.bytes()).enumerate() {
            if x != y {
                found = true;
                if len_s == len_t {
                    return s.as_bytes()[i + 1..] == t.as_bytes()[i + 1..];
                } else {
                    return s.as_bytes()[i + 1..] == t.as_bytes()[i..];
                }
            }
        }
        found || len_s - len_t == 1
    }
}

struct Solution {}

#[test]
fn test_is_one_edit_distance() {
    let s = "ab".to_string();
    let t = "acb".to_string();
    assert_eq!(Solution::is_one_edit_distance(s, t), true)
}

#[test]
fn test_is_one_edit_distance2() {
    let s = "cab".to_string();
    let t = "ad".to_string();
    assert_eq!(Solution::is_one_edit_distance(s, t), false)
}
