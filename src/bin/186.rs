// 186. 反转字符串中的单词 II

impl Solution {
    pub fn reverse_words(s: &mut Vec<char>) {
        let len = s.len();
        Self::reverse_m_n(0, len - 1, s);

        let mut ia = 0;
        for ib in 0..len {
            if s[ib].is_ascii_whitespace() || ib == len - 1 {
                if ib == len - 1 {
                    Self::reverse_m_n(ia, ib, s);
                } else {
                    Self::reverse_m_n(ia, ib - 1, s);
                }
                ia = ib + 1;
            }
        }
    }

    fn reverse_m_n(mut m: usize, mut n: usize, s: &mut Vec<char>) {
        while m < n {
            s.swap(m, n);
            m += 1;
            n -= 1;
        }
    }
}

struct Solution {}

#[test]
fn test_reverse_words() {
    let mut s: Vec<char> = vec![
        't', 'h', 'e', ' ', 's', 'k', 'y', ' ', 'i', 's', ' ', 'b', 'l', 'u', 'e',
    ];
    let exp = vec![
        'b', 'l', 'u', 'e', ' ', 'i', 's', ' ', 's', 'k', 'y', ' ', 't', 'h', 'e',
    ];
    Solution::reverse_words(&mut s);
    assert_eq!(s, exp)
}
