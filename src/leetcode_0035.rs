// Easy

// Ex1:
// input: nums = [1, 3, 5, 6] target: 5
// output: 2
// Ex2:
// input: nums = [1, 3, 5, 6]  target: 2
// output: 1
// Ex3:
// input: nums = [1, 3, 5, 6]  target: 7
// output: 4

pub fn search_insert1(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search(&target) {
        Ok(num) => return num as i32,
        Err(num) => return num as i32,
    }
}

pub fn search_insert2(nums: Vec<i32>, target: i32) -> i32 {
    nums.binary_search(&target).unwrap_or_else(|i| i) as i32
}

pub fn search_insert3(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
        let mid = left + ((right - left) / 2);
        if nums[mid] > target {
            right = mid - 1;
        } else if nums[mid] < target {
            left = mid + 1;
        } else {
            return mid as i32;
        }
    }
    return left as i32;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_search_insert1() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        assert_eq!(2, search_insert1(nums.clone(), target));
        let target = 2;
        assert_eq!(1, search_insert1(nums.clone(), target));
        let target = 7;
        assert_eq!(4, search_insert1(nums.clone(), target));
    }
    #[test]
    fn test_search_insert2() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        assert_eq!(2, search_insert2(nums.clone(), target));
        let target = 2;
        assert_eq!(1, search_insert2(nums.clone(), target));
        let target = 7;
        assert_eq!(4, search_insert2(nums.clone(), target));
    }
    #[test]
    fn test_search_insert3() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        assert_eq!(2, search_insert3(nums.clone(), target));
        let target = 2;
        assert_eq!(1, search_insert3(nums.clone(), target));
        let target = 7;
        assert_eq!(4, search_insert3(nums.clone(), target));
    }
}
