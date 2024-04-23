// Easy

// Ex1:
// input: s = "ab#c", t = "ad#c"
// output: true
// Ex2:
// input: s = "ab##", t = "c#d#"
// output: true
// Ex3:
// input: s = "a#c", t = "b"
// output: false

pub fn backspace_compare(s: String, t: String) -> bool {
    let builder = |s: String| -> String {
        let mut result = String::new();
        for c in s.chars() {
            if c != '#' {
                result.push(c);
            } else if !result.is_empty() {
                result.pop();
            }
        }
        result
    };
    builder(s) == builder(t)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_backspace_compare() {
        let s = "ab#c".to_string();
        let t = "ad#c".to_string();
        assert_eq!(true, backspace_compare(s, t));
        let s1 = "a#c".to_string();
        let t1 = "b".to_string();
        assert_eq!(false, backspace_compare(s1, t1));
    }
}
