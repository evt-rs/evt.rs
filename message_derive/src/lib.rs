extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Message)]
pub fn message_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap(); // TODO: use expect or panic

    impl_message_macro(&ast)
}

fn impl_message_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Message for #name {
            fn write() {}
        }
    };
    gen.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
