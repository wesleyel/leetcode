// 1056. 易混淆数

impl Solution {
    pub fn confusing_number(n: i32) -> bool {
        let (mut num, mut ans) = (n, 0);
        while num > 0 {
            match num % 10 {
                6 => ans = ans * 10 + 9,
                9 => ans = ans * 10 + 6,
                x @ (0 | 1 | 8) => ans = ans * 10 + x,
                _ => return false,
            }
            num /= 10;
        }
        ans != n
    }
}

struct Solution {}

#[test]
fn test_confusing_number() {
    assert_eq!(Solution::confusing_number(6), true);
    assert_eq!(Solution::confusing_number(89), true);
    assert_eq!(Solution::confusing_number(11), false);
}
