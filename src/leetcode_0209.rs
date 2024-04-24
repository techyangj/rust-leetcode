// Mid

// Ex1:
// input: target = 7, nums = [2,3,1,2,4,3]
// output: 2
// Ex2:
// input:  target = 4, nums = [1,4,4]
// output: 1
// Ex3:
// input:  target = 11, nums = [1,1,1,1,1,1,1,1]
// output: 0

pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let (mut result, mut subLength): (i32, i32) = (i32::MAX, 0);
    let (mut sum, mut i) = (0, 0);

    for (pos, val) in nums.iter().enumerate() {
        sum += val;
        while sum >= target {
            subLength = (pos - i + 1) as i32;
            if result > subLength {
                result = subLength;
            }
            sum -= nums[i];
            i += 1;
        }
    }
    if result == i32::MAX {
        return 0;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_sub_array_len() {
        let target = 7;
        let nums = vec![2, 3, 1, 2, 4, 3];
        assert_eq!(min_sub_array_len(target, nums), 2);
    }
}
