#[cfg(test)]
mod tests {
    use mutable_references::append_suffix;

    #[test]
    fn test_append_suffix() {
        let mut s = String::from("hello");
        append_suffix(&mut s, " world");
        assert_eq!(s, "hello world");

        append_suffix(&mut s, "!");
        assert_eq!(s, "hello world!");

        append_suffix(&mut s, "??");
        assert_eq!(s, "hello world!??");

        append_suffix(&mut s, " I love Rust!");
        assert_eq!(s, "hello world!?? I love Rust!");
    }
}
