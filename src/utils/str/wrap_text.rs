use textwrap::wrap;

pub fn wrap_text(text: &str, width: u16) -> (usize, String) {
    let lines = wrap(text, width as usize)
        .into_iter()
        .map(|line| line.into_owned())
        .collect::<Vec<_>>();

    (lines.len(), lines.join("\n"))
}
