// Easy

// Ex1:
// input: nums = [-1, 0, 3, 5, 9, 12] target: 9
// output: 4
// Ex2:
// input: nums = [-1, 0, 3, 5, 9, 12]  target: 2
// output: -1

use std::cmp::Ordering;

pub fn search1(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search(&target) {
        Ok(num) => num as i32,
        Err(_) => -1,
    }
}

pub fn search2(nums: Vec<i32>, target: i32) -> i32 {
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
    return -1;
}

pub fn search3(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
        let mid = left + ((right - left) / 2);
        match nums[mid].cmp(&target) {
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid - 1,
            Ordering::Equal => return mid as i32,
        }
    }
    return -1;

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_search1() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target1 = 9;
        assert_eq!(4, search1(nums.clone(), target1));
        let target2 = 2;
        assert_eq!(-1, search1(nums.clone(), target2));
    }
    #[test]
    fn test_search2() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target1 = 9;
        assert_eq!(4, search2(nums.clone(), target1));
        let target2 = 2;
        assert_eq!(-1, search2(nums.clone(), target2));
    }
    #[test]
    fn test_search3() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target1 = 9;
        assert_eq!(4, search3(nums.clone(), target1));
        let target2 = 2;
        assert_eq!(-1, search3(nums.clone(), target2));
    }
}
