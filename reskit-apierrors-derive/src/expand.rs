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
                        #ty::#ident => http_types::StatusCode::try_form(#status_code).unwrap(),
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
                        #ty::#ident => reskit_apierrors::pvlost::PVLost::try_from(#pvlost).unwrap(),
                    })
                }
                _ => None,
            }
        });
        Some(quote! {
            #[cfg(feature = "pvlost")]
            fn pvlost(&self) -> reskit_apierrors::pvlost::PVLost {
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
            impl #impl_generics reskit_apierrors::apierror::APIErrorMetas for #ty #ty_generics #where_clause {
                fn api_error_metas() -> Vec<&'static dyn reskit_apierrors::apierror::APIErrorMeta> {
                    vec![
                        #(#arms)*
                    ]
                }
            }
        })
    };

    let display_impl =  {
        Some(quote! {
            #[allow(unused_qualifications)]
            impl #impl_generics std::fmt::Display for #ty #ty_generics #where_clause {
                #[cfg(not(feature = "pvlost"))]
                fn fmt(&self, f: &mut std::fmt::Formatter) -> Result {
                    write!(f, "{}:{}:{}:{}", self.status_code(), self.system(), self.code(), self.message())  
                }
            
                #[cfg(feature = "pvlost")]
                fn fmt(&self, f: &mut std::fmt::Formatter) -> Result {
                    write!(f, "{}:{}:{}:{}:{}", self.status_code(), self.system(), self.code(), self.message(), self.pvlost() as u8)
                }
            }
        })
    };

    quote! {
        #[allow(unused_qualifications)]
        impl #impl_generics APIErrorMeta for #ty #ty_generics #where_clause {
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
