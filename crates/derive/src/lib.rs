use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(TryToHashMap)]
pub fn derive_try_to_hashmap(_input: TokenStream) -> TokenStream {
  quote! {}.into()
}
