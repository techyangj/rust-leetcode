// Easy

// Ex1:
// input: nums = [0,1,0,3,12]
// output: [1,3,12,0,0]
// Ex2:
// input:  nums = [0]
// output: [0]

pub fn move_zeroes1(nums: &mut Vec<i32>) {
    let mut slow_index = 0;
    for pos in 0..nums.len() {
        if nums[pos] != 0 {
            nums[slow_index] = nums[pos];
            if slow_index != pos {
                nums[pos] = 0;
            }
            slow_index += 1;
        }
    }
}

pub fn move_zeroes2(nums: &mut Vec<i32>) {
    let nums_length = nums.len();
    nums.retain(|&x| x != 0);
    for _i in 0..(nums_length - nums.len()) {
        nums.push(0);
    }
}

pub fn move_zeroes3(nums: &mut Vec<i32>) {
    let nums_length = nums.len();
    nums.retain(|&x| x != 0);
    nums.resize(nums_length, 0);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_move_zeroes1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        move_zeroes1(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
        let mut nums1 = vec![0];
        move_zeroes1(&mut nums1);
        assert_eq!(nums1, vec![0]);
    }
    #[test]
    fn test_move_zeroes2() {
        let mut nums = vec![0, 1, 0, 3, 12];
        move_zeroes2(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
        let mut nums1 = vec![0];
        move_zeroes2(&mut nums1);
        assert_eq!(nums1, vec![0]);
    }
    #[test]
    fn test_move_zeroes3() {
        let mut nums = vec![0, 1, 0, 3, 12];
        move_zeroes3(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
        let mut nums1 = vec![0];
        move_zeroes3(&mut nums1);
        assert_eq!(nums1, vec![0]);
    }
}
