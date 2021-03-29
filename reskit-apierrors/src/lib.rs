#[macro_use]
extern crate shadow_rs;

shadow!(build);

pub mod pvlost;
pub mod apierror;
pub mod globals;
pub mod errorspace;
pub mod builtin;
pub mod macros;

pub use pvlost::PVLost;
pub use apierror::{APIErrorMeta, APIError, APIErrorMetaEnum, CloneAPIErrorMeta};
pub use errorspace::Errorspace;
pub use builtin::BuiltinAPIErrorMeta;
pub use globals::{
    ERRORS,
    register_api_error_meta_enum,
    overwrite_api_error_meta_enum};

#[cfg(test)]
mod test {
    #[test]
    fn test_version() {
        shadow!(build);
        assert_eq!(build::PKG_VERSION, "0.1.0"); 
    }
}