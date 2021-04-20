use crate::attr::{self, Attrs};
use proc_macro2::Span;
use syn::{
    Data, DataEnum, DeriveInput, Error, Fields, Generics, Ident, Index, Member, Result,
    Type,
};

pub enum Input<'a> {
    Enum(Enum<'a>),
}

pub struct Enum<'a> {
    pub original: &'a DeriveInput,
    pub attrs: Attrs<'a>,
    pub ident: Ident,
    pub generics: &'a Generics,
    pub variants: Vec<Variant<'a>>,
}

pub struct Variant<'a> {
    pub original: &'a syn::Variant,
    pub attrs: Attrs<'a>,
    pub ident: Ident,
    pub fields: Vec<Field<'a>>,
}

pub struct Field<'a> {
    pub original: &'a syn::Field,
    pub attrs: Attrs<'a>,
    pub member: Member,
    pub ty: &'a Type,
}

impl<'a> Input<'a> {
    pub fn from_syn(node: &'a DeriveInput) -> Result<Self> {
        match &node.data {
            Data::Struct(_) => Err(Error::new_spanned(
                node,
                "struct as api error metas are not supported",
            )),
            Data::Enum(data) => Enum::from_syn(node, data).map(Input::Enum),
            Data::Union(_) => Err(Error::new_spanned(
                node,
                "union as api error metas are not supported",
            )),
        }
    }
}

impl<'a> Enum<'a> {
    fn from_syn(node: &'a DeriveInput, data: &'a DataEnum) -> Result<Self> {
        let attrs = attr::get(&node.attrs)?;
        let span = attrs.span().unwrap_or_else(Span::call_site);
        let variants = data
            .variants
            .iter()
            .map(|node| {
                let mut variant = Variant::from_syn(node, span)?;
                if let meta @ None = &mut variant.attrs.meta {
                    *meta = attrs.meta.clone();
                }
                Ok(variant)
            })
            .collect::<Result<_>>()?;
        Ok(Enum {
            original: node,
            attrs,
            ident: node.ident.clone(),
            generics: &node.generics,
            variants,
        })
    }
}

impl<'a> Variant<'a> {
    fn from_syn(node: &'a syn::Variant, span: Span) -> Result<Self> {
        let attrs = attr::get(&node.attrs)?;
        let span = attrs.span().unwrap_or(span);
        Ok(Variant {
            original: node,
            attrs,
            ident: node.ident.clone(),
            fields: Field::multiple_from_syn(&node.fields, span)?,
        })
    }
}

impl<'a> Field<'a> {
    fn multiple_from_syn(fields: &'a Fields, span: Span) -> Result<Vec<Self>> {
        fields
            .iter()
            .enumerate()
            .map(|(i, field)| Field::from_syn(i, field, span))
            .collect()
    }

    fn from_syn(i: usize, node: &'a syn::Field, span: Span) -> Result<Self> {
        Ok(Field {
            original: node,
            attrs: attr::get(&node.attrs)?,
            member: node.ident.clone().map(Member::Named).unwrap_or_else(|| {
                Member::Unnamed(Index {
                    index: i as u32,
                    span,
                })
            }),
            ty: &node.ty,
        })
    }
}

impl Attrs<'_> {
    pub fn span(&self) -> Option<Span> {
        if let Some(meta) = &self.meta {
            Some(meta.system.span())
        } else {
            None
        }
    }
}