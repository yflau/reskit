use proc_macro2::{TokenStream, Span};
use quote::{quote, ToTokens};
use syn::parse::ParseStream;
use syn::{
    Attribute, Ident, Error, LitInt, LitStr,
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
    attr.parse_args_with(|input: ParseStream| {
        let mut lit_system: Option<LitStr> = None;
        let mut lit_code: Option<LitStr> = None;
        let mut lit_message: Option<LitStr> = None;
        let mut lit_status_code: Option<LitInt> = None;
        let mut lit_pvlost: Option<LitInt> = None;

        let lookahead = input.lookahead1();
        while lookahead.peek(Ident) {
            let field = input.parse::<Ident>()?;
            match field.to_string().as_str() {
                "system" => {
                    if lit_system.is_some() {
                        return Err(Error::new_spanned(
                            attr,
                            "duplicate #[apierrormeta(system)] attribute",
                        ));
                    }
                    let _: Token![=] = input.parse()?;
                    let lit = input.parse::<LitStr>()?;
                    lit_system = Some(lit);
                },
                "code" => {
                    if lit_code.is_some() {
                        return Err(Error::new_spanned(
                            attr,
                            "duplicate #[apierrormeta(code)] attribute",
                        ));
                    }
                    let _: Token![=] = input.parse()?;
                    let lit = input.parse::<LitStr>()?;
                    lit_code = Some(lit);
                },
                "message" => {
                    if lit_message.is_some() {
                        return Err(Error::new_spanned(
                            attr,
                            "duplicate #[apierrormeta(message)] attribute",
                        ));
                    }
                    let _: Token![=] = input.parse()?;
                    let lit = input.parse::<LitStr>()?;
                    lit_message = Some(lit);
                },
                "status_code" => {
                    if lit_status_code.is_some() {
                        return Err(Error::new_spanned(
                            attr,
                            "duplicate #[apierrormeta(status_code)] attribute",
                        ));
                    }
                    let _: Token![=] = input.parse()?;
                    let lit  = input.parse::<LitInt>()?;
                    lit_status_code = Some(lit);
                },
                "pvlost" => {
                    if lit_pvlost.is_some() {
                        return Err(Error::new_spanned(
                            attr,
                            "duplicate #[apierrormeta(pvlost)] attribute",
                        ));
                    }
                    let _: Token![=] = input.parse()?;
                    let lit = input.parse::<LitInt>()?;
                    lit_pvlost = Some(lit);
                },
                _ => {
                    return Err(Error::new_spanned(
                        attr,
                        "unknown apierrormeta field",
                    ));
                }
            }
            match input.parse::<Token![,]>() {
                Ok(_) => continue,
                Err(_) => break,
            }
        }

        let meta = Meta {
            original: attr,
            system: lit_system.expect("system should not be None"),
            code: lit_code.expect("code should not be None"),
            message: lit_message.expect("message should not be None"),
            status_code: lit_status_code.expect("status_code should not be None"),
            pvlost: match lit_pvlost {
                Some(lit) => lit,
                None => LitInt::new("1", Span::call_site()), // NOTE: set default!
            },
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