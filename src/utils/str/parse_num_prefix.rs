pub fn parse_num_prefix(s: &str) -> Option<(u32, &str)> {
    let mut chars = s.chars().peekable();
    let mut num = String::new();

    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_digit() {
            num.push(ch);
            chars.next();
        } else {
            break;
        }
    }

    if !num.is_empty() {
        if let Ok(n) = num.parse::<u32>() {
            return Some((n, &s[num.len()..]));
        }
    }

    None
}
