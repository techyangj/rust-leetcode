// Easy

// Ex1:
// input: nums = [-4,-1,0,3,10]
// output: [0,1,9,16,100]
// Ex2:
// input: nums = [-7,-3,2,3,11]
// output: [4,9,9,49,121]

pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut slow_index = 0;
    let mut fast_index = nums.len() - 1;
    let mut result: Vec<i32> = Vec::new();
    while slow_index < fast_index {
        if nums[slow_index].abs() > nums[fast_index].abs() {
            result.insert(0, nums[slow_index].pow(2));
            slow_index += 1;
        } else {
            result.insert(0, nums[fast_index].pow(2));
            fast_index -= 1;
        }
    }
    result.insert(0, nums[fast_index].pow(2));
    result
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sorted_squares() {
        let nums = vec![-4, -1, 0, 3, 10];
        assert_eq!(vec![0, 1, 9, 16, 100], sorted_squares(nums));
        let nums1 = vec![-7, -3, 2, 3, 11];
        assert_eq!(vec![4, 9, 9, 49, 121], sorted_squares(nums1));
    }
}
