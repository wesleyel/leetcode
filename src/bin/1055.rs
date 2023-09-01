// 1055. 形成字符串的最短路径

impl Solution {
    pub fn shortest_way(source: String, target: String) -> i32 {
        let (mut cnt, lt) = (0, target.len());
        let mut it = 0;
        while it < lt {
            let ori = it;
            for s in source.chars() {
                if it < lt && target.as_bytes()[it] == s as u8 {
                    it += 1;
                }
            }
            if ori == it {
                return -1;
            }
            cnt += 1;
        }
        cnt
    }
}

struct Solution {}

#[test]
fn test_shortest_way() {
    let s = "abc".to_string();
    let t = "abcbc".to_string();
    assert_eq!(Solution::shortest_way(s, t), 2);
}
