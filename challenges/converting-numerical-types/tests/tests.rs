#[cfg(test)]
mod tests {
    use converting_numerical_types::*;
    use syntest::Syntest;

    #[test]
    fn test_converts_numbers() {
        assert_eq!(numerical_type_conversion(42i32), 42u32);
        assert_eq!(numerical_type_conversion(0i32), 0u32);
        assert_eq!(numerical_type_conversion(-1i32), 4294967295u32);
        assert_eq!(numerical_type_conversion(1_000_000i32), 1_000_000u32);
    }

    #[test]
    fn test_uses_as() {
        {
            let syntest = Syntest::new("numerical_type_conversion", "src/lib.rs");

            let visited = syntest
                .as_visitor
                .usages()
                .first()
                .expect("No `as` usage found");

            assert_eq!(visited, "n");
        }

        // ------------------------------------------------------
        // Some other local tests that is not related to the user
        {
            let code = r"
            fn convert(n: i32) -> u32 {
                let n = n as u32;

                n
            }

            fn convert_2(n: i32) -> u32 {
                n as u32
            }

            fn convert_3(n: i32) -> u32 {
                return n as u32; 
            }
            ";

            let syntest = Syntest::from_code("convert", code);

            assert_eq!(syntest.as_visitor.usages().len(), 1);
            assert!(syntest.as_visitor.usages().iter().all(|x| x == "n"));

            let syntest = Syntest::from_code("convert_2", code);
            assert_eq!(syntest.as_visitor.usages().len(), 1);
            assert!(syntest.as_visitor.usages().iter().all(|x| x == "n"));

            let syntest = Syntest::from_code("convert_3", code);
            assert_eq!(syntest.as_visitor.usages().len(), 1);
            assert!(syntest.as_visitor.usages().iter().all(|x| x == "n"));
        }
    }
}
