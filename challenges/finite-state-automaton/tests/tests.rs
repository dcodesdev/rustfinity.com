#[cfg(test)]
mod tests {
    use finite_state_automaton::*;

    #[test]
    fn test_recognize_pattern_valid() {
        assert_eq!(recognize_pattern("abbbc"), true);
        assert_eq!(recognize_pattern("ac"), true);
    }

    #[test]
    fn test_recognize_pattern_invalid() {
        assert_eq!(recognize_pattern("abbbd"), false);
        assert_eq!(recognize_pattern(""), false);
    }

    #[test]
    fn test_recognize_pattern_edge_cases() {
        assert_eq!(recognize_pattern("abbbbbc"), true);
        assert_eq!(recognize_pattern("a"), false);
        assert_eq!(recognize_pattern("abc"), true);
    }
}
