// This file is for internal use.

pub fn trim_whitespace(s: &str) -> String {
    let mut new_str = String::new();
    let mut prev_is_space = true;

    for ch in s.chars() {
        if ch == ' ' || ch == '\u{a0}' {
            if !prev_is_space {
                new_str.push(' ');
            }
            prev_is_space = true;
        } else {
            new_str.push(ch);
            prev_is_space = false;
        }
    }
    new_str.trim().to_owned()
}
