use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(TestMacro)]
pub fn test_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_first_macro(&ast)
}

fn impl_first_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl TestMacro for #name {
            fn first_macro() {
                println!("Hello, Test Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
