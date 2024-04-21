// Easy

// Ex1:
// input: x = 4
// output: 2
// Ex2:
// input: x = 8
// output: 2
pub fn my_sqrt(x: i32) -> i32 {
    let mut left = 0;
    let mut right = x;
    let mut ans = -1;
    while left <= right {
        let mid = left + ((right - left) /  2);
        let mid2 = mid as i64 * mid as i64;
        if mid2 <= x as i64 {
            ans = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    return ans; 
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_my_sqrt() {
        let x = 4;
        assert_eq!(my_sqrt(x), 2);
        let x = 8;
        assert_eq!(my_sqrt(x), 2);
    }
}