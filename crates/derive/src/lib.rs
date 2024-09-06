use proc_macro::TokenStream;
use quote::quote;

#[cfg(test)]
mod tests;

#[proc_macro_derive(TryToHashMap)]
pub fn derive_try_to_hashmap(_input: TokenStream) -> TokenStream {
  quote! {}.into()
}
