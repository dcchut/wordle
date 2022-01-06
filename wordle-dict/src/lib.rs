mod private {
    include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
}

// CLion can't see through the include! above so re-export here.
/// This is a set of lowercase English words sourced from <https://github.com/dwyl/english-words>.
///
/// # Example
///
/// ```rust
/// use wordle_dict::WORDS;
///
/// assert_eq!(WORDS.len(), 370_102);
/// assert_eq!(WORDS.contains("apple"), true);
/// assert_eq!(WORDS.contains("notarealword"), false);
/// ```
pub static WORDS: &phf::Set<&'static str> = &private::_WORDS;
