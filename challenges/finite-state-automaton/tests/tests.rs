#[cfg(test)]
mod tests {
    use finite_state_automaton::*;

    #[test]
    fn test_recognize_pattern_valid() {
        assert_eq!(recognize_pattern("abbbc"), true);
        assert_eq!(recognize_pattern("ac"), true);
        assert_eq!(recognize_pattern("ab"), false);
        assert_eq!(recognize_pattern("abbc"), true);
        assert_eq!(recognize_pattern("abbbbbc"), true);
    }

    #[test]
    fn test_recognize_pattern_invalid() {
        assert_eq!(recognize_pattern("abbbd"), false);
        assert_eq!(recognize_pattern(""), false);
        assert_eq!(recognize_pattern("a"), false);
        assert_eq!(recognize_pattern("abbd"), false);
        assert_eq!(recognize_pattern("abbbcc"), false);
    }

    #[test]
    fn test_recognize_pattern_edge_cases() {
        assert_eq!(recognize_pattern("abbbbbc"), true);
        assert_eq!(recognize_pattern("a"), false);
        assert_eq!(recognize_pattern("abc"), true);
        assert_eq!(recognize_pattern("abbc"), true);
        assert_eq!(recognize_pattern("ab"), false);
    }

    #[test]
    fn test_recognize_pattern_long_input() {
        let long_input_valid = "a".to_owned() + "b".repeat(434).as_str() + "c";
        let long_input_invalid = "a".to_owned() + "b".repeat(333).as_str() + "d";
        assert_eq!(recognize_pattern(&long_input_valid), true);
        assert_eq!(recognize_pattern(&long_input_invalid), false);
        assert_eq!(recognize_pattern(&(long_input_valid.clone() + "c")), false);
    }

    #[test]
    fn test_recognize_pattern_random_cases() {
        assert_eq!(recognize_pattern("abbbbbbbbbc"), true);
        assert_eq!(recognize_pattern("abbbbbbbbd"), false);
        assert_eq!(recognize_pattern("aabbcc"), false);
        assert_eq!(recognize_pattern("abbbc"), true);
        assert_eq!(recognize_pattern("acc"), false);
    }
}
