extern crate awoo;
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

#[proc_macro_derive(Awoo)]
pub fn awoo_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the token stream into a syntax tree.
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = ast.ident;

    // Build the impl
    let expanded = quote! {
        impl Awoo for #name {
            fn awoo() {
                println!("I am a {}! Awoo!", stringify!(#name));
            }
        }
    };

    // Return the generated impl
    expanded.into()
}
