use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct PlainText {
    value: String,
}

impl PlainText {
    fn new(value: String) -> Self {
        PlainText { value }
    }
}

impl From<&str> for PlainText {
    fn from(s: &str) -> Self {
        PlainText {
            value: s.to_string(),
        }
    }
}

impl Display for PlainText {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.value.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::quickcheck;

    quickcheck! {
        fn should_wrap_a_string(text: String) -> bool {
            let value = PlainText::new(text.clone());
            value.to_string() == text
        }
    }
}
