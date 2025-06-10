pub fn first_char_str(s: &str) -> Option<&str> {
    s.char_indices().next().map(|(i, _)| {
        let next = s.char_indices().nth(1).map(|(j, _)| j).unwrap_or(s.len());
        &s[i..next]
    })
}
