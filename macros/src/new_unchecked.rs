use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::DeriveInput;

pub struct NewUnchecked {
    ident: syn::Ident,
    generics: syn::Generics,
    ty: syn::Type,
}

fn get_type(data: &syn::Data, ident: &syn::Ident) -> Result<syn::Type, syn::Error> {
    match data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => {
            if fields.len() > 1 {
                return Err(syn::Error::new_spanned(
                    fields.iter().nth(1).unwrap(),
                    "NewUnchecked must have exactly one field",
                ));
            }

            let field = fields.iter().next().ok_or_else(|| {
                syn::Error::new_spanned(ident, "NewUnchecked must have exactly one field")
            })?;
            Ok(field.ty.clone())
        }
        _ => Err(syn::Error::new_spanned(
            ident,
            "NewUnchecked can only be derived for structs",
        )),
    }
}

impl NewUnchecked {
    pub fn from_derive_input(input: &DeriveInput) -> Result<Self, syn::Error> {
        let ident = input.ident.clone();
        let generics = input.generics.clone();
        let data = input.data.clone();
        let ty = get_type(&data, &ident)?;
        Ok(Self {
            ident,
            generics,
            ty,
        })
    }
}

impl ToTokens for NewUnchecked {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let generics = &self.generics;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let ty = &self.ty;

        let impl_newtype = quote! {
            impl #impl_generics #ident #ty_generics #where_clause {
                pub unsafe fn new_unchecked(value: #ty) -> Self {
                    #ident(value)
                }
            }
        };

        tokens.extend(impl_newtype);
    }
}