use std::{any::Any, collections::HashMap};

use derive_util::Input;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro]
pub fn hashmap(input: TokenStream) -> TokenStream {
  let input: Input = parse_macro_input!(input);
  (|| -> Result<HashMap<&str, &dyn Any>, ()> {
    let hashmap: HashMap<&str, &dyn Any> = HashMap::new();
    Ok(hashmap)
  })()
  .unwrap();
  println!("te");
  quote! {}.into()
}

#[proc_macro_derive(HashMap)]
pub fn derive_try_to_hashmap(input: TokenStream) -> TokenStream {
  let input: DeriveInput = parse_macro_input!(input);
  match impl_try_to_hashmap(&input) {
    Ok(v) => v,
    Err(e) => e.to_compile_error().into(),
  }
}

fn impl_try_to_hashmap(input: &DeriveInput) -> Result<TokenStream, syn::Error> {
  let ident = &input.ident;
  let (impl_generics, _, where_clause) = &input.generics.split_for_impl();
  Ok(
    quote! {
      use derive_util::{TryToHashMap};
      impl #impl_generics TryToHashMap for #ident<'_> #where_clause {
        fn try_to_hashmap(&self) -> Result<(), ()> {
          println!("try");
          crate::hashmap!(self);
          Ok(())
        }
      }
    }
    .into(),
  )
}
