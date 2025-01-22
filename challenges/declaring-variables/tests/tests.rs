use declaring_variables::*;
use syntest::quote;

#[test]
fn test_calculate_area() {
    let area = calculate_area();

    assert!(
        area > 0,
        "The `calculate_area` function must return a value greater than 0"
    )
}

#[test]
fn test_prints_values() {
    {
        let code = quote! {
            use declaring_variables::*;

            let width = 10;
            let height = 50;

            prints_values(width, height);
        };

        let result = syntest::create_bin_and_run(&code);

        assert_eq!(
            result.stdout(),
            "The width is: 10\nThe height is: 50\n",
            "The `prints_values` must not be modified for the tests to pass\nPlease reset the function to its original state"
        );
    }

    {
        let code = quote! {
            use declaring_variables::*;

            calculate_area();
        };

        let result = syntest::create_bin_and_run(&code);

        assert!(
            result.stdout().contains("The width is"),
            "The `prints_values must be used"
        );
        assert!(
            result.stdout().contains("The height is"),
            "The `prints_values must be used"
        );
    }
}
