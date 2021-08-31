use anyhow::{anyhow, bail, Result};
use quote::{format_ident, quote};
use syn::parse::Parser;

pub fn create_entity(ast: &syn::DeriveInput) -> Result<proc_macro2::TokenStream> {
    match &ast.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields) => {
                if fields.named.is_empty() {
                    bail!("Entity cannot be derived from Struct without fields");
                }
                if !&ast.generics.params.is_empty() {
                    bail!("Entity cannot be derived from Struct with Generic Type Parameters");
                }
                let entity_struct = create_derived_entity_struct(&ast, &data_struct, &fields);
                let from_impl =
                    implement_from_trait(&entity_struct.ident, &fields.named, &ast.ident);
                let entity_impl = implement_entity_trait(&entity_struct.ident);
                let updateable_impl =
                    implement_updateable_trait(&entity_struct.ident, &fields.named, &ast.ident);
                let entity = quote!(
                    #[derive(Clone, Debug)]
                    #entity_struct
                    #from_impl
                    #entity_impl
                    #updateable_impl
                );
                Ok(entity)
            }
            syn::Fields::Unnamed(_) => Err(anyhow!(
                "Entity cannot be derived from Struct with unnamed fields"
            )),
            syn::Fields::Unit => Err(anyhow!("Entity cannot be derived from Unit Struct")),
        },
        syn::Data::Enum(_) => Err(anyhow!("Entity cannot be derived from Enum")),
        syn::Data::Union(_) => Err(anyhow!("Entity cannot be derived from Union")),
    }
}

fn create_derived_entity_struct(
    ast: &syn::DeriveInput,
    data_struct: &syn::DataStruct,
    named_fields: &syn::FieldsNamed,
) -> syn::DeriveInput {
    let entity_ident = format_ident!("{}Entity", &ast.ident);
    let entity_fields = entity_fields(named_fields);
    syn::DeriveInput {
        ident: entity_ident,
        data: syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(entity_fields),
            ..data_struct.clone()
        }),
        ..ast.clone()
    }
}

fn entity_fields(named_fields: &syn::FieldsNamed) -> syn::FieldsNamed {
    let mut entity_fields = named_fields.clone();
    vec![
        quote! { pub id: ddd_core::Uuid },
        quote! { pub created_on: ddd_core::DateTime<ddd_core::Utc> },
        quote! { pub created_by: String },
        quote! { pub updated_on: ddd_core::DateTime<ddd_core::Utc> },
        quote! { pub updated_by: String },
    ]
        .into_iter()
        .map(|f| syn::Field::parse_named.parse2(f).unwrap())
        .for_each(|f| entity_fields.named.push(f));
    entity_fields
}

fn implement_from_trait(
    from_ident: &syn::Ident,
    to_fields: &syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
    to_ident: &syn::Ident,
) -> proc_macro2::TokenStream {
    let transformation: Vec<_> = to_fields
        .iter()
        .filter_map(|i| i.ident.as_ref())
        .map(|ident| quote! {#ident : input.#ident})
        .collect();

    quote!(
        impl From<#from_ident> for #to_ident {
            fn from(input: #from_ident) -> Self {
                Self {
                    #(#transformation),*
                }
            }
        }
    )
}

fn implement_entity_trait(entity_ident: &syn::Ident) -> proc_macro2::TokenStream {
    quote!(
        impl ddd_core::Entity<ddd_core::Uuid> for #entity_ident {
            fn id(&self) -> &ddd_core::Uuid {
                &self.id
            }

            fn created_on(&self) -> &ddd_core::DateTime<ddd_core::Utc> {
                &self.created_on
            }

            fn created_by(&self) -> &str {
                &self.created_by
            }

            fn updated_on(&self) -> &ddd_core::DateTime<ddd_core::Utc> {
                &self.updated_on
            }
            fn updated_by(&self) -> &str {
                &self.updated_by
            }
        }
    )
}

fn implement_updateable_trait(
    entity_ident: &syn::Ident,
    input_fields: &syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
    input_ident: &syn::Ident,
) -> proc_macro2::TokenStream {
    let updates: Vec<_> = input_fields
        .iter()
        .filter_map(|i| i.ident.as_ref())
        .map(|ident| quote! { self.#ident = data.#ident})
        .collect();

    quote!(
        impl ddd_core::UpdateableWith<#input_ident> for #entity_ident {
            fn update(&mut self, data: #input_ident) -> &mut Self {
                  #(#updates;)*;
                  self
            }
        }
    )
}
