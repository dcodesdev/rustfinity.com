#[cfg(test)]

mod tests {
    use primitive_data_types::*;
    use syntest::{parse_quote, Pat, Stmt, Syntest};

    #[test]
    fn test_compiles() {
        data_types();
    }

    #[test]
    fn test_annotations() {
        let syntest = Syntest::new("data_types", "./src/lib.rs");

        let pats_actual = syntest.get_pats();
        let stmts_expected: [Stmt; 4] = [
            parse_quote! {
              let x: u8 = 42;
            },
            parse_quote! {
              let y: f64 = 3.14;
            },
            parse_quote! {
              let z: bool = true;
            },
            parse_quote! {
              let a: char = 'R';
            },
        ];

        let mut expected_count = 0;
        for (pat_actual, stmt_expected) in pats_actual.iter().zip(stmts_expected.iter()) {
            if let Stmt::Local(local_expected) = stmt_expected {
                if let Pat::Type(type_expected) = &local_expected.pat {
                    if let Pat::Ident(ident_expected) = &*type_expected.pat {
                        let pat_expected = &local_expected.pat;
                        assert_eq!(
                            pat_actual, pat_expected,
                            "Wrong type annotated for {}",
                            ident_expected.ident
                        );
                        expected_count += 1;
                    }
                }
            }
        }

        assert_eq!(expected_count, stmts_expected.len());
    }
}
