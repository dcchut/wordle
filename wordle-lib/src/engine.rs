use crate::{Inference, Overlap};
use std::collections::HashSet;

pub struct Engine<const LEN: usize> {
    words: Vec<&'static str>,
}

impl<const LEN: usize> Engine<LEN> {
    pub fn new<I: IntoIterator<Item = &'static str>>(iter: I) -> Self {
        Self {
            words: iter.into_iter().filter(|w| w.len() == LEN).collect(),
        }
    }

    /// Determines all valid words and the overlap scores between them.
    pub fn evaluate(&self, inf: &[Inference]) -> Vec<(&'static str, Overlap)> {
        let valid_words: Vec<_> = self
            .words
            .iter()
            .filter(|&&w| inf.iter().all(|f| f.filter(w)))
            .copied()
            .collect();

        // We collect everything into two buckets: one based on "has this character"
        // and one based on "has this character at this position".
        let mut character_buckets: [HashSet<usize>; 26] =
            array_init::array_init(|_| HashSet::new());
        let mut positional_buckets: [[HashSet<usize>; 26]; LEN] =
            array_init::array_init(|_| array_init::array_init(|_| HashSet::new()));

        for (i, word) in valid_words.iter().enumerate() {
            for (j, c) in lowercase_iter(word).enumerate() {
                character_buckets[c].insert(i);
                positional_buckets[j][c].insert(i);
            }
        }

        valid_words
            .into_iter()
            .map(|w| {
                let mut overlap = Overlap::default();
                let mut seen = [false; 26];

                for (i, c) in lowercase_iter(w).enumerate() {
                    if !seen[c] {
                        overlap.partial += character_buckets[c].len();
                        seen[c] = true;
                    }
                    overlap.total += positional_buckets[i][c].len();
                }

                (w, overlap)
            })
            .collect()
    }
}

fn lowercase_iter(w: &'static str) -> impl Iterator<Item = usize> {
    w.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .flat_map(|c| c.to_lowercase())
        .map(|c| ((c as u8) - b'a') as usize)
}

#[cfg(test)]
mod tests {
    use super::Engine;
    use crate::{Inference, InferenceKind, Overlap};

    #[test]
    fn inference_engine() {
        let words = ["apple", "banan", "taple", "agora", "doggo"];
        let engine = Engine::<5>::new(words);

        // Filter out any words that don't have an `a` in them.
        let words = engine.evaluate(&vec![Inference::new('a', 2, InferenceKind::Present)]);
        assert_eq!(words.len(), 4);

        // Overlaps:
        //
        // apple: 5 total, 4 letters.
        // banan: 0 total, 1 letters.
        // taple: 3 total, 4 letters.
        // agora: 1 total, 1 letters.
        // doggo: doesn't contain an A.
        //
        assert_eq!(words[0], ("apple", Overlap::new(9, 10)));

        // Overlaps:
        //
        // apple: 0 total, 1 letters.
        // banan: 5 total, 3 letters.
        // taple: 1 total, 1 letters.
        // agora: 0 total, 1 letters.
        // doggo: doesn't contain an A.
        //
        assert_eq!(words[1], ("banan", Overlap::new(6, 6)));
    }
}
