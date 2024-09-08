use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ToHashMap)]
pub fn derive_try_to_hashmap(input: TokenStream) -> TokenStream {
  let input: DeriveInput = parse_macro_input!(input);
  match impl_try_to_hashmap(&input) {
    Ok(v) => v,
    Err(e) => e.to_compile_error().into(),
  }
}

fn impl_try_to_hashmap(input: &DeriveInput) -> Result<TokenStream, syn::Error> {
  let ident = &input.ident;
  let (impl_generics, _ty_generics, where_clause) = &input.generics.split_for_impl();
  let mut vec = Vec::new();
  let struct_data = match &input.data {
    syn::Data::Struct(v) => v,
    _ => return Err(syn::Error::new_spanned(&input.ident, "")),
  };

  for field in &struct_data.fields {
    let ident = field.ident.as_ref().unwrap();
    let key = format!("{}", ident);
    vec.push(quote! {
      hashmap.insert(#key,&self.#ident);
    });
  }
  let quote = quote! {
    use std::any::Any;
    use std::collections::HashMap;
    use derive_util::TryToHashMap;
    impl #impl_generics TryToHashMap for #ident #where_clause {
      fn try_to_hashmap(&self) -> HashMap<&str, &dyn Any> {
        let mut hashmap = HashMap::<&str, &dyn Any>::new();
        #(#vec)*
        hashmap
      }
    }
  };
  println!("{}", format!("{}", quote));
  Ok(quote.into())
}
