#[test]
fn main() {
    let res = is_palindrome(0);
    println!("{res}")
}
fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    } else if x < 10 {
        return true;
    } else {
        let s = x.to_string();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i <= j {
            if s.chars().nth(i) != s.chars().nth(j) {
                return false;
            }
            i += 1;
            j -= 1;
        }
    }
    return true;
}
