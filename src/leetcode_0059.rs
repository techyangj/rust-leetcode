// mid

// Ex1:
// input: n = 3
// output: [[1,2,3],[8,9,4],[7,6,5]]
// Ex2:
// input: n = 1
// output: [[1]]

pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let mut res = vec![vec![0; n as usize]; n as usize];
    let (mut startX, mut startY, mut offset): (usize, usize, usize) = (0, 0, 1);
    let mut loop_idx = n / 2;
    let mid: usize = loop_idx as usize;
    let (mut i, mut j): (usize, usize) = (0, 0);
    let mut count = 1;
    while loop_idx > 0 {
        i = startX;
        j = startY;
        while j < (startY + (n as usize) - offset) {
            res[i][j] = count;
            count += 1;
            j += 1;
        }
        while i < (startX + (n as usize) - offset) {
            res[i][j] = count;
            count += 1;
            i += 1;
        }
        while j > startY {
            res[i][j] = count;
            count += 1;
            j -= 1;
        }
        while i > startX {
            res[i][j] = count;
            count += 1;
            i -= 1;
        }
        startX += 1;
        startY += 1;
        loop_idx -= 1;
        offset += 2;
    }
    if n % 2 == 1 {
        res[mid][mid] = count;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_matrix() {
        let mix = vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        assert_eq!(mix, generate_matrix(3))
    }
}
