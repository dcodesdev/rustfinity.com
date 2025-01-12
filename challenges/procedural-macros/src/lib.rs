use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Describe)]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    // Only proceed if the struct has named fields
    let fields = if let Data::Struct(data_struct) = input.data {
        if let Fields::Named(named_fields) = data_struct.fields {
            named_fields.named
        } else {
            return TokenStream::new(); // Ignore non-named fields
        }
    } else {
        return TokenStream::new(); // Ignore non-struct data
    };

    let field_names = fields.iter().map(|f| f.ident.as_ref().unwrap());
    let field_names_as_strings = field_names.clone().map(|name| name.to_string());
    let generated = quote! {
        impl Describe for #struct_name {
            fn describe(&self) -> String {
                let fields: Vec<String> = vec![
                    #(format!("{}: {:?}", #field_names_as_strings, self.#field_names)),*
                ];
                format!("{} {{ {} }}", stringify!(#struct_name), fields.join(", "))
            }
        }
    };

    generated.into()
}

// Example Test
// #[test]
// fn test_example() {
//     let person = Person {
//         name: "Alice".to_string(),
//         age: 30,
//     };

//     assert_eq!(person.describe(), "Person { name: \"Alice\", age: 30 }");
// }
