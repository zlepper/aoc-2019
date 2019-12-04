use std::collections::HashMap;

fn main() {
    brute_force()
}

fn brute_force() {
    let mut valid_count = 0;
    for pw in 273025..767253 {
        if is_valid(&pw.to_string()) {
            valid_count += 1;
        }
    }

    println!("Valid count: {}", valid_count)
}

fn is_valid(pw: &str) -> bool {
    let chars: Vec<&str> = pw.split("").filter(|s| !s.is_empty()).collect();

    let mut double_counter = HashMap::new();

    let mut seen_double = false;
    for i in 0..(chars.len() - 1) {
        let this = chars[i];
        let next = chars[i + 1];

        if this.parse::<i32>().unwrap() > next.parse().unwrap() {
            return false
        }
    }

    for c in chars {
        double_counter.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }

    for count in double_counter.values() {
        if *count == 2 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(is_valid("123789"), false);
    }

    #[test]
    fn examples2() {
        assert_eq!(is_valid("223450"), false);
    }

    #[test]
    fn examples3() {
        assert_eq!(is_valid("111111"), false);
    }

    #[test]
    fn example2() {
        assert_eq!(is_valid("112233"), true);
    }
    #[test]
    fn example3() {
        assert_eq!(is_valid("123444"), false);
    }
    #[test]
    fn example4() {
        assert_eq!(is_valid("111122"), true);
    }
}