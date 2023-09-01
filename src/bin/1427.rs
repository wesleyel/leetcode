// 1427. 字符串的左右移

impl Solution {
    pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
        let mut index = shift.iter().fold(0, |acc, x| acc + i32::pow(-1, x[0] as u32) * x[1]);
        let len = s.len() as i32;
        index = (index % len + len) % len;
        format!("{}{}", &s[index as usize..], &s[..index as usize])
    }
}

struct Solution {}

macro_rules!vec2d {
    [ $( [ $( $d:expr ),* ] ),* ] => {
        vec![
            $(
                vec![$($d),*],
            )*
        ]
    }
}

#[test]
fn test_string_shift() {
    let src = "abc".to_string();
    let exp: String = "cab".to_string();
    assert_eq!(
        Solution::string_shift(src, vec![vec![0, 1], vec![1, 2]]),
        exp
    );
}

#[test]
fn test_string_shift2() {
    let src = "yisxjwry".to_string();
    let exp: String = "yisxjwry".to_string();
    assert_eq!(
        Solution::string_shift(
            src,
            vec![
                vec![1, 8],
                vec![1, 4],
                vec![1, 3],
                vec![1, 6],
                vec![0, 6],
                vec![1, 4],
                vec![0, 2],
                vec![0, 1]
            ]
        ),
        exp
    );
}

#[test]
fn test_string_shift3() {
    let src = "xqgwkiqpif".to_string();
    let exp: String = "qpifxqgwki".to_string();
    assert_eq!(
        Solution::string_shift(
            src,
            vec2d![
                [1, 4],
                [0, 7],
                [0, 8],
                [0, 7],
                [0, 6],
                [1, 3],
                [0, 1],
                [1, 7],
                [0, 5],
                [0, 6]
            ]
        ),
        exp
    );
}
