use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// #[proc_macro]
// pub fn hashmap(input: TokenStream) -> TokenStream {
//   quote! {
//     (|| {

//     })();
//   }
//   .into()
// }

#[proc_macro_derive(HashMap)]
pub fn derive_try_to_hashmap(input: TokenStream) -> TokenStream {
  let input: DeriveInput = parse_macro_input!(input);
  match impl_try_to_hashmap(&input) {
    Ok(v) => v,
    Err(e) => e.to_compile_error().into(),
  }
}

fn impl_try_to_hashmap(input: &DeriveInput) -> Result<TokenStream, syn::Error> {
  let data = match &input.data {
    syn::Data::Struct(v) => v,
    _ => return Err(syn::Error::new_spanned(input, "")),
  };
  let field = match &data.fields {
    syn::Fields::Named(v) => v,
    _ => return Err(syn::Error::new_spanned(input, "")),
  };
  let struct_name = &input.ident;
  let (impl_generics, _, where_clause) = &input.generics.split_for_impl();
  Ok(
    quote! {
      use derive_util::TryToHashMap;
      impl #impl_generics TryToHashMap for #struct_name<'_> #where_clause {}
    }
    .into(),
  )
}
