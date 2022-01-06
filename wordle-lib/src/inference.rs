#[derive(Copy, Clone, Debug)]
pub struct Inference {
    c: char,
    positive: bool,
    source: usize,
    exact: bool,
}

impl Inference {
    pub fn new(c: char, positive: bool, source: usize, exact: bool) -> Self {
        Self {
            c,
            positive,
            source,
            exact,
        }
    }

    pub fn filter(&self, w: &'static str) -> bool {
        if self.exact {
            let c = w.chars().nth(self.source).unwrap();
            if self.positive {
                self.c == c
            } else {
                self.c != c
            }
        } else if self.positive {
            w.contains(self.c)
        } else {
            !w.contains(self.c)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Inference;
    use crate::overlap::Overlap;

    #[test]
    fn inference_filter_positive_partial() {
        // There should be an 'a' _somewhere_ in the word.
        let inference = Inference::new('a', true, 1, false);

        assert_eq!(inference.filter("cat"), true);
        assert_eq!(inference.filter("dottttttg"), false);
        assert_eq!(inference.filter("tttttttta"), true);
        assert_eq!(inference.filter("aottttttttto"), true);
    }

    #[test]
    fn inference_filter_positive_total() {
        // There should be an 'a' at a specific position in the word.
        let inference = Inference::new('a', true, 2, true);

        assert_eq!(inference.filter("ammmmmm"), false);
        assert_eq!(inference.filter("zzzzzzz"), false);
        assert_eq!(inference.filter("thamm"), true);
        assert_eq!(inference.filter("ooaoa"), true);
    }

    #[test]
    fn inference_filter_negative_partial() {
        // There isn't an 'a' at a given position in the word.
        let inference = Inference::new('a', false, 1, true);

        assert_eq!(inference.filter("abbbbb"), true);
        assert_eq!(inference.filter("ccccc"), true);
        assert_eq!(inference.filter("tazzzz"), false);
        assert_eq!(inference.filter("aaaaa"), false);
        assert_eq!(inference.filter("pppa"), true);
    }

    #[test]
    fn inference_filter_negative_total() {
        // There is no 'a' anywhere in the word.
        let inference = Inference::new('a', false, 0, false);

        assert_eq!(inference.filter("bbbbbbb"), true);
        assert_eq!(inference.filter("abbbbb"), false);
        assert_eq!(inference.filter("bbbbba"), false);
        assert_eq!(inference.filter("cdefghi"), true);
    }
}
