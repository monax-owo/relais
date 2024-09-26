use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// #[derive(Editable)]
// struct Data {
//   #[immutable]
//   field: String,
// }

#[proc_macro_derive(Editable, attributes(mutable, immutable))]
pub fn derive_editable(input: TokenStream) -> TokenStream {
  let input: DeriveInput = parse_macro_input!(input);
  match impl_editable(&input) {
    Ok(v) => v,
    Err(e) => e.to_compile_error().into(),
  }
}

fn impl_editable(_input: &DeriveInput) -> Result<TokenStream, syn::Error> {
  let quote = quote! {};
  Ok(quote.into())
}
