//! Retrieve the numeric value for a key from a dictionary where a value is
//! either a number or a *reference* to another key.
//!
//! Type choices:
//! - Rust has no `String | i32` union type, so "number or reference" is a
//!   tagged `enum Value`. `match` on it is exhaustive and checked at compile
//!   time — replacing the draft's runtime `type(val) == "String"` test.
//! - Visited keys are a `HashSet<String>` (one type parameter — the element),
//!   not the draft's `HashSet<String, bool>`; a set already answers "seen it?".
//! - `resolve` returns `Option<i64>`: `None` covers both a missing key and a
//!   reference cycle (`local -> default -> local`) that never reaches a number.

use std::collections::{HashMap, HashSet};

/// A dictionary entry: a concrete number, or a pointer to another key.
/// Encodes the draft's `String | i32` as a proper sum type.
#[derive(Debug, Clone)]
enum Value {
    Num(i64),
    Ref(String),
}

/// Resolve `key` to its numeric value, following references transitively.
///
/// Returns `None` when the key is missing, or when the reference chain forms a
/// cycle and therefore never bottoms out at a number.
fn resolve(map: &HashMap<String, Value>, key: &str) -> Option<i64> {
    let mut seen = HashSet::new();
    let mut current = key;

    loop {
        // A key we've already visited means we're going in circles.
        if !seen.insert(current.to_string()) {
            return None;
        }

        match map.get(current)? {
            Value::Num(n) => return Some(*n),
            Value::Ref(next) => current = next,
        }
    }
}

fn main() {
    // Timeout (seconds) per deployment stage; some stages point at another.
    let config = HashMap::from([
        ("prod".to_string(), Value::Num(2)),
        ("test".to_string(), Value::Num(5)),
        ("local".to_string(), Value::Ref("default".to_string())),
        ("default".to_string(), Value::Ref("local".to_string())),
        ("staging".to_string(), Value::Ref("test".to_string())),
    ]);

    for key in ["prod", "test", "staging", "local", "missing"] {
        println!("{key:>8} -> {:?}", resolve(&config, key));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> HashMap<String, Value> {
        HashMap::from([
            ("prod".to_string(), Value::Num(2)),
            ("test".to_string(), Value::Num(5)),
            ("local".to_string(), Value::Ref("default".to_string())),
            ("default".to_string(), Value::Ref("local".to_string())),
            ("staging".to_string(), Value::Ref("test".to_string())),
        ])
    }

    #[test]
    fn direct_number() {
        assert_eq!(resolve(&config(), "prod"), Some(2));
        assert_eq!(resolve(&config(), "test"), Some(5));
    }

    #[test]
    fn single_reference() {
        assert_eq!(resolve(&config(), "staging"), Some(5));
    }

    #[test]
    fn cyclic_reference_returns_none() {
        assert_eq!(resolve(&config(), "local"), None);
        assert_eq!(resolve(&config(), "default"), None);
    }

    #[test]
    fn missing_key_returns_none() {
        assert_eq!(resolve(&config(), "missing"), None);
    }

    #[test]
    fn longer_chain() {
        let map = HashMap::from([
            ("a".to_string(), Value::Ref("b".to_string())),
            ("b".to_string(), Value::Ref("c".to_string())),
            ("c".to_string(), Value::Num(42)),
        ]);
        assert_eq!(resolve(&map, "a"), Some(42));
    }
}
