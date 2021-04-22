use crate::ast::{Enum, Input};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

pub fn derive(node: &DeriveInput) -> Result<TokenStream> {
    let input = Input::from_syn(node)?;
    input.validate()?;
    Ok(match input {
        Input::Enum(input) => impl_enum(input),
    })
}

fn impl_enum(input: Enum) -> TokenStream {
    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let system_method = {
        let arms = input.variants.iter().map(|variant| {
            match &variant.attrs.meta {
                Some(meta) => {
                    let ident = &variant.ident;
                    let system = &meta.system;
                    Some(quote! {
                        #ty::#ident => #system,
                    })
                }
                None => None,
            }
        });
        Some(quote! {
            fn system(&self) -> &str {
                match self {
                    #(#arms)*
                }
            }
        })
    };

    let code_method = {
        let arms = input.variants.iter().map(|variant| {
            match &variant.attrs.meta {
                Some(meta) => {
                    let ident = &variant.ident;
                    let code = &meta.code;
                    Some(quote! {
                        #ty::#ident => #code,
                    })
                }
                None => None,
            }
        });
        Some(quote! {
            fn code(&self) -> &str {
                match self {
                    #(#arms)*
                }
            }
        })
    };

    let message_method = {
        let arms = input.variants.iter().map(|variant| {
            match &variant.attrs.meta {
                Some(meta) => {
                    let ident = &variant.ident;
                    let message = &meta.message;
                    Some(quote! {
                        #ty::#ident => #message,
                    })
                }
                None => None,
            }
        });
        Some(quote! {
            fn message(&self) -> &str {
                match self {
                    #(#arms)*
                }
            }
        })
    };

    let status_code_method = {
        let arms = input.variants.iter().map(|variant| {
            match &variant.attrs.meta {
                Some(meta) => {
                    let ident = &variant.ident;
                    let status_code = &meta.status_code;
                    Some(quote! {
                        #ty::#ident => http_types::StatusCode::try_from(#status_code).unwrap(),
                    })
                }
                None => None,
            }
        });
        Some(quote! {
            fn status_code(&self) -> http_types::StatusCode {
                match self {
                    #(#arms)*
                }
            }
        })
    };

    #[cfg(not(feature = "pvlost"))]
    let pvlost_method: Option<TokenStream> = None;

    #[cfg(feature = "pvlost")]
    let pvlost_method = {
        let arms = input.variants.iter().map(|variant| {
            match &variant.attrs.meta {
                Some(meta) => {
                    let ident = &variant.ident;
                    let pvlost = &meta.pvlost;
                    Some(quote! {
                        #ty::#ident => crate::PVLost::try_from(#pvlost).unwrap(),
                    })
                }
                _ => None,
            }
        });
        Some(quote! {
            fn pvlost(&self) -> crate::PVLost {
                match self {
                    #(#arms)*
                }
            }
        })
    };

    let apierrormetas_impl = {
        let arms = input.variants.iter().map(|variant| {
            let ident = &variant.ident;
            quote! {
                &Self::#ident,
            }
        });
        Some(quote! {
            #[allow(unused_qualifications)]
            impl #impl_generics crate::APIErrorMetas for #ty #ty_generics #where_clause {
                fn api_error_metas() -> Vec<&'static dyn crate::APIErrorMeta> {
                    vec![
                        #(#arms)*
                    ]
                }
            }
        })
    };

    #[cfg(not(feature = "pvlost"))]
    let display_impl =  {
        Some(quote! {
            #[allow(unused_qualifications)]
            impl #impl_generics std::fmt::Display for #ty #ty_generics #where_clause {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{}:{}:{}:{}", self.status_code(), self.system(), self.code(), self.message())  
                }
            }
        })
    };

    #[cfg(feature = "pvlost")]
    let display_impl =  {
        Some(quote! {
            #[allow(unused_qualifications)]
            impl #impl_generics std::fmt::Display for #ty #ty_generics #where_clause {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{}:{}:{}:{}:{}", self.status_code(), self.system(), self.code(), self.message(), self.pvlost() as u8)
                }
            }
        })
    };

    quote! {
        use std::convert::TryFrom;
        #[allow(unused_qualifications)]
        impl #impl_generics crate::APIErrorMeta for #ty #ty_generics #where_clause {
            #system_method
            #code_method
            #message_method
            #status_code_method
            #pvlost_method
        }
        #apierrormetas_impl
        #display_impl
    }
}
