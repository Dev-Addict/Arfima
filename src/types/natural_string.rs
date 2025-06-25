use std::{cmp::Ordering, fmt};

#[derive(Debug, Clone)]
pub struct NaturalString(String);

impl NaturalString {
    pub fn new(s: &str) -> Self {
        Self(s.to_string())
    }

    pub fn str(&self) -> &str {
        &self.0
    }

    fn cmp_natural(a: &str, b: &str) -> Ordering {
        let mut a_chars = a.chars().peekable();
        let mut b_chars = b.chars().peekable();

        while a_chars.peek().is_some() || b_chars.peek().is_some() {
            match (a_chars.peek(), b_chars.peek()) {
                (Some(ac), Some(bc)) if ac.is_ascii_digit() && bc.is_ascii_digit() => {
                    let mut a_num = String::new();
                    while let Some(c) = a_chars.peek() {
                        if c.is_ascii_digit() {
                            a_num.push(*c);
                            a_chars.next();
                        } else {
                            break;
                        }
                    }

                    let mut b_num = String::new();
                    while let Some(c) = b_chars.peek() {
                        if c.is_ascii_digit() {
                            b_num.push(*c);
                            b_chars.next();
                        } else {
                            break;
                        }
                    }

                    let a_val = a_num.parse::<u64>().unwrap_or(0);
                    let b_val = b_num.parse::<u64>().unwrap_or(0);
                    let ord = a_val.cmp(&b_val);
                    if ord != Ordering::Equal {
                        return ord;
                    }
                }
                (Some(ac), Some(bc)) => {
                    let ord = ac.cmp(bc);
                    if ord != Ordering::Equal {
                        return ord;
                    }
                    a_chars.next();
                    b_chars.next();
                }
                (Some(_), None) => return Ordering::Greater,
                (None, Some(_)) => return Ordering::Less,
                (None, None) => break,
            }
        }

        Ordering::Equal
    }
}

impl PartialEq for NaturalString {
    fn eq(&self, other: &Self) -> bool {
        Self::cmp_natural(&self.0, &other.0) == Ordering::Equal
    }
}

impl Eq for NaturalString {}

impl PartialOrd for NaturalString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NaturalString {
    fn cmp(&self, other: &Self) -> Ordering {
        Self::cmp_natural(&self.0, &other.0)
    }
}

impl fmt::Display for NaturalString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<String> for NaturalString {
    fn from(value: String) -> Self {
        Self(value)
    }
}
