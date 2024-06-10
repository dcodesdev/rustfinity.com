#[cfg(test)]
mod tests {
    use constants::*;
    use syntest::Syntest;

    #[test]
    fn compiles() {
        assert_eq!(main(), 100, "Expected main() to return 100");
    }

    #[test]
    fn test_constants() {
        let syntest = Syntest::new("main", "src/lib.rs");

        let const_to_exist = ["MAX_SIZE"];
        let constants = syntest.constant.constants();

        let mut success = false;
        for const_name in const_to_exist.iter() {
            constants.iter().for_each(|con| {
                if con.ident.to_string() != *const_name {
                    return;
                }

                assert_eq!(
                    con.ty,
                    syntest::parse_quote! { i32 },
                    "Expected type i32 for constant MAX_SIZE"
                );
                assert_eq!(
                    con.expr,
                    syntest::parse_quote! { 100 },
                    "Expected value 100 for constant MAX_SIZE"
                );

                success = true;
            })
        }

        assert!(success, "Expected constant MAX_SIZE to exist");
    }
}
