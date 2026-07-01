//! Given a `&str`, return its longest whitespace-separated word.
//!
//! Type choices:
//! - Returns `Option<&str>`, not `&str`. The result is a *slice into the
//!   input*, and empty/whitespace-only input has no longest word — `None`
//!   makes that a type-level outcome instead of panicking on `words[0]`.
//! - No intermediate `Vec<&str>`: `split_whitespace()` yields an iterator we
//!   reduce directly with `max_by_key`.

/// Longest word in a single string. Ties resolve to the last such word
/// (`max_by_key` keeps the last maximum).
fn longest_word(s: &str) -> Option<&str> {
    s.split_whitespace().max_by_key(|word| word.len())
}

/// Longest word across two strings.
///
/// The `'a` lifetime is the point of this signature: the returned slice borrows
/// from *one of* the inputs, so both `s1` and `s2` must share `'a` and outlive
/// the result. Writing `-> &str` with no `'a` (as the draft did) wouldn't
/// compile — the compiler can't infer which input the borrow comes from.
fn longest_word_couple<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    [longest_word(s1), longest_word(s2)]
        .into_iter()
        .flatten()
        .max_by_key(|word| word.len())
}

fn main() {
    let sentence = "the quick brown fox jumped";
    println!("longest in {sentence:?} -> {:?}", longest_word(sentence));

    let a = "a lazy dog";
    let b = "an energetic caterpillar";
    println!(
        "longest across {a:?} and {b:?} -> {:?}",
        longest_word_couple(a, b)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_longest() {
        assert_eq!(longest_word("the quick brownish fox"), Some("brownish"));
    }

    #[test]
    fn ties_resolve_to_last() {
        // "quick" and "brown" are both 5 chars; max_by_key keeps the last.
        assert_eq!(longest_word("the quick brown fox"), Some("brown"));
    }

    #[test]
    fn single_word() {
        assert_eq!(longest_word("hello"), Some("hello"));
    }

    #[test]
    fn empty_and_whitespace() {
        assert_eq!(longest_word(""), None);
        assert_eq!(longest_word("   \t\n "), None);
    }

    #[test]
    fn ignores_extra_whitespace() {
        assert_eq!(longest_word("  a   bb  ccc "), Some("ccc"));
    }

    #[test]
    fn couple_picks_global_longest() {
        assert_eq!(
            longest_word_couple("a lazy dog", "an energetic caterpillar"),
            Some("caterpillar")
        );
    }

    #[test]
    fn couple_handles_one_empty_side() {
        assert_eq!(longest_word_couple("", "solo"), Some("solo"));
        assert_eq!(longest_word_couple("", ""), None);
    }
}
