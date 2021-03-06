#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum InferenceKind {
    /// The character does not appear in the word at all.
    AbsentGlobal,
    /// The character does not appear in the word at the same position.
    AbsentLocal,
    /// The character appears at some position (but not this one).
    Present,
    /// The character appears at the position.
    Correct,
    /// The character appears this many times.
    Count(usize),
    /// The character appears at least this many times.
    AtLeast(usize),
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Inference {
    c: char,
    position: usize,
    kind: InferenceKind,
}

impl Inference {
    pub fn new(c: char, position: usize, kind: InferenceKind) -> Self {
        Self {
            c: c.to_ascii_lowercase(),
            position,
            kind,
        }
    }

    pub fn char(&self) -> char {
        self.c
    }

    pub fn filter(&self, w: &'static str) -> bool {
        let c = w.chars().nth(self.position).unwrap().to_ascii_lowercase();
        match self.kind {
            InferenceKind::AbsentGlobal => !w.contains(self.c),
            InferenceKind::AbsentLocal => self.c != c,
            InferenceKind::Present => self.c != c && w.contains(self.c),
            InferenceKind::Correct => self.c == c,
            InferenceKind::Count(n) => {
                w.chars()
                    .filter(|&q| q.to_ascii_lowercase() == self.c)
                    .count()
                    == n
            }
            InferenceKind::AtLeast(n) => {
                w.chars()
                    .filter(|&q| q.to_ascii_lowercase() == self.c)
                    .count()
                    >= n
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Inference, InferenceKind};

    #[test]
    fn inference_filter_at_least() {
        let inference = Inference::new('u', 1, InferenceKind::AtLeast(2));

        assert_eq!(inference.filter("TUUES"), true);
        assert_eq!(inference.filter("trout"), false);
        assert_eq!(inference.filter("tatou"), false);
    }

    #[test]
    fn inference_filter_positive_partial() {
        // There should be an 'a' _somewhere_ in the word.
        let inference = Inference::new('a', 1, InferenceKind::Present);

        // note that `cat` is not a match because InferenceKind::Present won't match
        // an `a` at the same position as the inference.
        assert_eq!(inference.filter("cat"), false);

        // `tca` _is_ a match because it has an `a` not at position 1.
        assert_eq!(inference.filter("tca"), true);
        assert_eq!(inference.filter("dottttttg"), false);
        assert_eq!(inference.filter("tttttttta"), true);
        assert_eq!(inference.filter("aottttttttto"), true);
    }

    #[test]
    fn inference_filter_positive_total() {
        // There should be an 'a' at a specific position in the word.
        let inference = Inference::new('a', 2, InferenceKind::Correct);

        assert_eq!(inference.filter("ammmmmm"), false);
        assert_eq!(inference.filter("zzzzzzz"), false);
        assert_eq!(inference.filter("thamm"), true);
        assert_eq!(inference.filter("ooaoa"), true);
    }

    #[test]
    fn inference_filter_negative_partial() {
        // There isn't an 'a' at a given position in the word.
        let inference = Inference::new('a', 1, InferenceKind::AbsentLocal);

        // contains an `a` but not at position 1.
        assert_eq!(inference.filter("abbbbb"), true);
        assert_eq!(inference.filter("ccccc"), true);
        assert_eq!(inference.filter("tazzzz"), false);
        assert_eq!(inference.filter("aaaaa"), false);
        assert_eq!(inference.filter("pppa"), true);
    }

    #[test]
    fn inference_filter_negative_total() {
        // There is no 'a' anywhere in the word.
        let inference = Inference::new('a', 0, InferenceKind::AbsentGlobal);

        assert_eq!(inference.filter("bbbbbbb"), true);
        assert_eq!(inference.filter("abbbbb"), false);
        assert_eq!(inference.filter("bbbbba"), false);
        assert_eq!(inference.filter("cdefghi"), true);
    }

    #[test]
    fn inference_filter_count() {
        // There is exactly two `a`'s in the word.
        let inference = Inference::new('a', 2, InferenceKind::Count(2));

        assert_eq!(inference.filter("bbbb"), false);
        assert_eq!(inference.filter("abbb"), false);
        assert_eq!(inference.filter("aabb"), true);
        assert_eq!(inference.filter("bbaa"), true);
        assert_eq!(inference.filter("aaab"), false);
        assert_eq!(inference.filter("bbba"), false);
    }
}
