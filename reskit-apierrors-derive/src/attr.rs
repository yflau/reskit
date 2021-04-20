use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::ParseStream;
use syn::{
    Attribute, Error, Ident, LitInt, LitStr,
    Result, Token,
};

pub struct Attrs<'a> {
    pub meta: Option<Meta<'a>>,
}

#[derive(Clone)]
pub struct Meta<'a> {
    pub original: &'a Attribute,
    pub system: LitStr,
    pub code: LitStr,
    pub message: LitStr,
    pub status_code: LitInt,

    #[cfg(feature = "pvlost")]
    pub pvlost: LitInt,
}

pub fn get(input: &[Attribute]) -> Result<Attrs> {
    let mut attrs = Attrs {
        meta: None,
    };
    for attr in input {
        if attr.path.is_ident("apierrormeta") {
            parse_apierrormeta_attribute(&mut attrs, attr)?;
        }
    }
    Ok(attrs)
}

fn parse_apierrormeta_attribute<'a>(attrs: &mut Attrs<'a>, attr: &'a Attribute) -> Result<()> {
    syn::custom_keyword!(system);
    syn::custom_keyword!(code);
    syn::custom_keyword!(message);
    syn::custom_keyword!(status_code);
    syn::custom_keyword!(pvlost);
    attr.parse_args_with(|input: ParseStream| {
        let mut system: Option<LitStr> = None;
        let mut code: Option<LitStr> = None;
        let mut message: Option<LitStr> = None;
        let mut status_code: Option<LitInt> = None;

        #[cfg(feature = "pvlost")]
        let mut pvlost: Option<LitInt> = None;

        let lookahead = input.lookahead1();
        while lookahead.peek(Ident) {
            if let Some(_) = input.parse::<Option<system>>()? {
                if system.is_some() {
                    return Err(Error::new_spanned(
                        attr,
                        "duplicate #[apierrormeta(system)] attribute",
                    ));
                }
                let _: Token![=] = input.parse()?;
                let lit = input.parse::<LitStr>()?;
                system = Some(lit);
                continue
            }
            if let Some(_) = input.parse::<Option<code>>()? {
                if code.is_some() {
                    return Err(Error::new_spanned(
                        attr,
                        "duplicate #[apierrormeta(code)] attribute",
                    ));
                }
                let _: Token![=] = input.parse()?;
                let lit = input.parse::<LitStr>()?;
                code = Some(lit);
                continue
            }
            if let Some(_) = input.parse::<Option<message>>()? {
                if message.is_some() {
                    return Err(Error::new_spanned(
                        attr,
                        "duplicate #[apierrormeta(message)] attribute",
                    ));
                }
                let _: Token![=] = input.parse()?;
                let lit = input.parse::<LitStr>()?;
                message = Some(lit);
                continue
            }
            if let Some(_) = input.parse::<Option<status_code>>()? {
                if status_code.is_some() {
                    return Err(Error::new_spanned(
                        attr,
                        "duplicate #[apierrormeta(status_code)] attribute",
                    ));
                }
                let _: Token![=] = input.parse()?;
                let lit  = input.parse::<LitInt>()?;
                status_code = Some(lit);
                continue
            }

            #[cfg(feature = "pvlost")]
            if let Some(_) = input.parse::<Option<pvlost>>()? {
                if pvlost.is_some() {
                    return Err(Error::new_spanned(
                        attr,
                        "duplicate #[apierrormeta(pvlost)] attribute",
                    ));
                }
                let _: Token![=] = input.parse()?;
                let lit = input.parse::<LitInt>()?;
                pvlost = Some(lit);
                continue
            }
        }

        let meta = Meta {
            original: attr,
            system: system.unwrap(),
            code: code.unwrap(),
            message: message.unwrap(),
            status_code: status_code.unwrap(),

            #[cfg(feature = "pvlost")]
            pvlost: pvlost.unwrap(),
        };
    
        if attrs.meta.is_some() {
            return Err(Error::new_spanned(attr,"only one #[apierrormeta(...)] attribute is allowed"));
        }

        attrs.meta = Some(meta);
        Ok(())
    })
}

impl ToTokens for Meta<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! { &self });
    }
}