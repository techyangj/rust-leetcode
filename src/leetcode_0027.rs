// easy

// Ex1:
// input: nums = [3,2,2,3], val = 3
// output: 2, nums = [2,2]
// Ex2:
// input: nums = [0,1,2,2,3,0,4,2], val = 2
// output: 5, nums = [0,1,3,0,4]

pub fn remove_element1(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&num| num != val);
    nums.len() as i32
}

pub fn remove_element2(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut slow_index = 0;
    for pos in 0..nums.len() {
        if val != nums[pos] {
            nums[slow_index] = nums[pos];
            slow_index += 1;
        }
    }
    slow_index as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_element1() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        assert_eq!(2, remove_element1(&mut nums, val));

        let mut nums1 = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val1 = 2;
        assert_eq!(5, remove_element1(&mut nums1, val1));
    }
    #[test]
    fn test_remove_element2() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        assert_eq!(2, remove_element2(&mut nums, val));

        let mut nums1 = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val1 = 2;
        assert_eq!(5, remove_element2(&mut nums1, val1));
    }
}
