use std::fmt::Display;

pub trait TextPadding {
    fn pad_left(&self, p: &str, c: usize) -> String
    where
        Self: Display,
    {
        format!("{}{}", p.repeat(c), self)
    }

    fn pad_right(&self, p: &str, c: usize) -> String
    where
        Self: Display,
    {
        format!("{}{}", self, p.to_string().repeat(c))
    }

    fn pad(&self, p: &str, c: usize) -> String
    where
        Self: Display,
    {
        format!(
            "{}{}{}",
            p.to_string().repeat(c),
            self,
            p.to_string().repeat(c)
        )
    }

    fn p(&self) -> String
    where
        Self: Display,
    {
        format!(
            "{}{}{}",
            " ".to_string().repeat(1),
            self,
            " ".to_string().repeat(1)
        )
    }
}

impl TextPadding for &str {}

impl TextPadding for String {}

#[cfg(test)]
mod test {
    use super::TextPadding;

    #[test]
    fn padding_left_4() {
        let want = "    lorem";
        let have = "lorem".pad_left(" ", 4);
        assert_eq!(want, have)
    }

    #[test]
    fn padding_left_2() {
        let want = "  lorem";
        let have = "lorem".pad_left(" ", 2);
        assert_eq!(want, have)
    }

    #[test]
    fn padding_left_1() {
        let want = " lorem";
        let have = "lorem".pad_left(" ", 1);
        assert_eq!(want, have)
    }

    #[test]
    fn padding_right_1() {
        let want = "lorem ";
        let have = "lorem".pad_right(" ", 1);
        assert_eq!(want, have)
    }

    #[test]
    fn padding_right_2() {
        let want = "lorem  ";
        let have = "lorem".pad_right(" ", 2);
        assert_eq!(want, have)
    }

    #[test]
    fn padding_right_4() {
        let want = "lorem    ";
        let have = "lorem".pad_right(" ", 4);
        assert_eq!(want, have)
    }

    #[test]
    fn pad_both_sides_4() {
        let want = "    lorem    ";
        let have = "lorem".pad(" ", 4);
        assert_eq!(want, have)
    }

    #[test]
    fn pad_both_sides_2() {
        let want = "  lorem  ";
        let have = "lorem".pad(" ", 2);
        assert_eq!(want, have)
    }

    #[test]
    fn pad_both_sides_1() {
        let want = " lorem ";
        let have = "lorem".pad(" ", 1);
        assert_eq!(want, have)
    }
}
