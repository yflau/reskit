use crate::ast::{Enum, Field, Input, Variant};
use crate::attr::Attrs;
use syn::{Error, Result};

impl Input<'_> {
    pub(crate) fn validate(&self) -> Result<()> {
        match self {
            Input::Enum(input) => input.validate(),
        }
    }
}

impl Enum<'_> {
    fn validate(&self) -> Result<()> {
        check_non_variant_attrs(&self.attrs)?;
        for variant in &self.variants {
            variant.validate()?;
        }
        Ok(())
    }
}

impl Variant<'_> {
    fn validate(&self) -> Result<()> {
        if self.attrs.meta.is_none() {
            return Err(Error::new_spanned(
                self.original,
                "missing #[error(transparent)] attribute",
            ));
        }
        Ok(())
    }
}

impl Field<'_> {
    fn _validate(&self) -> Result<()> {
        if let Some(meta) = &self.attrs.meta {
            return Err(Error::new_spanned(
                meta.original,
                "not expected here; the #[apierrormeta(...)] attribute belongs on top of variant",
            ));
        }
        Ok(())
    }
}

fn check_non_variant_attrs(attrs: &Attrs) -> Result<()> {
    if let Some(meta) = &attrs.meta {
        return Err(Error::new_spanned(
            meta,
            "not expected here; the #[apierrormeta] attribute belongs on a specific variant",
        ));
    }
    Ok(())
}