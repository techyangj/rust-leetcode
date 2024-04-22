// easy

// Ex1:
// input: nums = [1,1,2]
// output: 2, nums = [1,2,_]
// Ex2:
// input: nums = [0,0,1,1,1,2,2,3,3,4]
// output: 5, nums = [0,1,2,3,4]

pub fn remove_duplicates1(nums: &mut Vec<i32>) -> i32 {
    let mut slow_index = 0;
    for pos in 1..nums.len() {
        if nums[slow_index] != nums[pos] {
            slow_index += 1;
            nums[slow_index] = nums[pos];
        }
    }
    (slow_index + 1) as i32
}

pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_duplicates1() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(2, remove_duplicates1(&mut nums));
        let mut nums1 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(5, remove_duplicates1(&mut nums1));
    }
    #[test]
    fn test_remove_duplicates2() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(2, remove_duplicates2(&mut nums));
        let mut nums1 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(5, remove_duplicates2(&mut nums1));
    }
}
