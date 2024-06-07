use declaring_variables::*;
use syntest::Syntest;

#[test]
fn test_calculate_area() {
    assert_eq!(calculate_area(), 50);
}

#[test]
fn test_variables() {
    let syntest = Syntest::from("./src/lib.rs");

    // expect all vars to be used
    let vars = syntest.variables("calculate_area");
    vars.iter().for_each(|var| {
        assert_eq!(var.is_used(), true);
    });

    let variables_to_exist = ["height", "width"];
    variables_to_exist.iter().for_each(|&v| {
        let var = syntest
            .var_details("calculate_area", v)
            .expect(&format!("Variable {} does not exist", v));

        assert_eq!(var.name, v);
        assert_eq!(var.used, true);
    });
}
