pub fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// pub fn trim_whitespace(s: &str) -> String {
//     let mut new_str = s.trim().to_owned();
//     let mut prev = ' ';
//     new_str.retain(|ch| {
//         let result = ch != ' ' || prev != ' ';
//         prev = ch;
//         result
//     });
//     new_str
// }

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
