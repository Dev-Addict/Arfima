pub fn all_but_first(s: &str) -> &str {
    s.chars().next().map(|c| &s[c.len_utf8()..]).unwrap_or("")
}
