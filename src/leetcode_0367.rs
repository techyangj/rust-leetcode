// Easy

// Ex1:
// input: num = 16
// output: true
// Ex2:
// input: num = 14
// output: false

pub fn is_perfect_square(num: i32) -> bool {
    let mut left = 0;
    let mut right = num;
    let mut result = false;

    while left <= right {
        let mid = left + ((right - left) / 2);
        let mid2 = mid as i64 * mid as i64;
        if mid2 < num as i64 {
            result = false;
            left = mid + 1;
        } else if mid2 > num as i64 {
            result = false;
            right = mid - 1;
        } else {
            result = true;
            return result;
        }
    }
    return result;
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_perfect_square() {
        let num = 16;
        assert_eq!(true, is_perfect_square(num));
        let num = 14;
        assert_eq!(false, is_perfect_square(num));
    }
}
