// Regex crate https://crates.io/crates/regex does not implement PartialEq.  Wrap and implement it
// ourselves.

use crate::error::*;

pub struct Regex(regex::Regex);

impl Regex {
    pub fn new(str: &str) -> Result<Option<Self>> {
        let pattern = match str.strip_prefix('/').and_then(|str| str.strip_suffix('/')) {
            Some(pat) => pat,
            None => return Ok(None),
        };
        match regex::Regex::new(pattern) {
            Ok(regex) => Ok(Some(Regex(regex))),
            Err(_) => Err(InvalidRegex(str.to_string())),
        }
    }

    pub fn is_match(&self, str: &str) -> bool {
        self.0.is_match(str)
    }
}

impl PartialEq for Regex {
    fn eq(&self, other: &Regex) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        for (left, right, expect) in &[
            ("/^foo/", "/^foo/", true),
            ("/foo/", "/foo/", true),
            ("/bar$/", "/bar$/", true),
            ("/^foo/", "/^bar/", false),
            ("/foo/", "/bar/", false),
            ("/bar$/", "/^bar/", false),
        ] {
            assert_eq!(
                Regex::new(left).unwrap() == Regex::new(right).unwrap(),
                *expect,
            );
        }
    }

    #[test]
    fn is_match() {
        for (regex, text, expect) in &[
            ("/^foo/", "foobar", true),
            ("/foo/", "foobar", true),
            ("/bar$/", "foobar", true),
            ("/^foo/", "barfoo", false),
            ("/foo/", "bar", false),
            ("/bar$/", "barfoo", false),
        ] {
            assert_eq!(Regex::new(regex).unwrap().unwrap().is_match(text), *expect);
        }
    }
}
