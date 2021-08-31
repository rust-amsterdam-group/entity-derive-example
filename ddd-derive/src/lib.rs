extern crate proc_macro;

use proc_macro_error::abort;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

mod entity;

#[proc_macro_derive(Entity)]
#[proc_macro_error]
pub fn entity(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match entity::create_entity(&input) {
        Ok(stream) => stream.into(),
        Err(e) => abort! { input,
                e.to_string();
                note = "Entity may only be derived from a Struct with named fields";
                help = "Try changing the item to a Struct with named fields";
        },
    }
}
