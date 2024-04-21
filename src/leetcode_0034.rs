// Middle

// Ex1:
// input: nums = [5, 7, 7, 8, 8, 10] target: 8
// output: [3, 4]
// Ex2:
// input: nums = [5, 7, 7, 8, 8, 10]  target: 6
// output: [-1, -1]
// Ex3:
// input: nums = []  target: 0
// output: [-1, -1]

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() == 0 || nums[0] > target || nums[nums.len() - 1] < target {
        return vec![-1, -1];
    }
    let left = search_binary(&nums, target, true);
    let right = search_binary(&nums, target, false) - 1;
    if nums[left as usize] == target && nums[right as usize] == target {
        vec![left as i32, right as i32]
    } else {
        vec![-1, -1]
    }
}

pub fn search_binary(nums: &Vec<i32>, target: i32, lower: bool) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut result = nums.len() as i32;
    while left <= right {
        let mid = left + ((right - left) / 2);
        if nums[mid] > target || (lower && nums[mid] == target) {
            right = mid - 1;
            result = mid as i32;
        } else {
            left = mid + 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_search_range() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 8;
        assert_eq!(search_range(nums.clone(), target), vec![3, 4]);
        let target = 6;
        assert_eq!(search_range(nums.clone(), target), vec![-1, -1]);

        let nums2 = vec![];
        let target2 = 0;
        assert_eq!(search_range(nums2.clone(), target2), vec![-1, -1]);
    }
}
