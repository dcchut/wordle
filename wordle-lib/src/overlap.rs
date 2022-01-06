use std::collections::HashSet;

/// Represents the total overlap between a word (called the `source`) and all other words (`targets`).
/// The two components are:
///
/// ## total
///
/// This represents the total number of characters among the targets that
/// match the source at the corresponding position in the word.
///
/// ## partial
///
/// This represents the total number of character among the targets that
/// match a character in the source at any position.
///
/// # Example
///
/// Given the words:
/// - `"cat"`,
/// - `"cot"`,
/// - `"toc"`,
///
/// with `"cat"` as the source we would have a `total` of five (three contributed from `"cat"`,
/// two from `"cot"` and zero from `"toc"`) and a `partial` of seven (three contributed from `"cat"`,
/// two from `"cot"` and two from `"toc"`):
///
/// ```rust
/// use wordle_lib::Overlap;
///
/// let words = ["cat", "cot", "toc"];
/// let overlap: Overlap = words.into_iter().map(|w| Overlap::from_words("cat", w)).sum();
///
/// assert_eq!(overlap, Overlap::new(5, 7));
/// ```
///
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Overlap {
    pub total: usize,
    pub partial: usize,
}

impl std::ops::Add for Overlap {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            total: self.total + rhs.total,
            partial: self.partial + rhs.partial,
        }
    }
}

impl std::iter::Sum for Overlap {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), std::ops::Add::add)
    }
}

impl Overlap {
    pub fn new(total: usize, letters: usize) -> Self {
        Self {
            total,
            partial: letters,
        }
    }

    fn _from_words(l: &str, r: &str) -> Self {
        assert_eq!(l.len(), r.len());
        let mut overlap = Overlap::new(0, 0);
        let mut seen = HashSet::with_capacity(l.len());

        for (cl, cr) in l.chars().zip(r.chars()) {
            if cl == cr {
                overlap.total += 1;
            }
            if l.contains(cr) && seen.insert(cr) {
                overlap.partial += 1;
            }
        }

        overlap
    }

    pub fn from_words<L: AsRef<str>, R: AsRef<str>>(l: L, r: R) -> Self {
        Self::_from_words(l.as_ref(), r.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::Overlap;

    #[test]
    fn overlap_sum() {
        let overlaps = vec![Overlap::new(5, 4), Overlap::new(9, 2), Overlap::new(3, 5)];

        let sum = overlaps.into_iter().sum::<Overlap>();
        assert_eq!(sum, Overlap::new(17, 11));
    }
}
