/// 一个简单的加法函数
///
/// # 参数
/// * `a` - 第一个加数
/// * `b` - 第二个加数
///
/// # 返回值
/// 返回两个数的和
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 一个简单的减法函数
///
/// # 参数
/// * `a` - 被减数
/// * `b` - 减数
///
/// # 返回值
/// 返回两个数的差
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
    }
}
