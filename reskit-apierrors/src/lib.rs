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
pub use apierror::{APIErrorMeta, APIErrorMetas, APIError, APIErrorMetaEnum};
pub use errorspace::Errorspace;
pub use builtin::Builtin;
pub use globals::{
    ERRORSPACES,
    new_errorspace,
    register_errorspace,
    clone_errorspace,
    register_api_error_metas,
    overwrite_api_error_metas,
    adapt, force};

#[cfg(test)]
mod tests {
    #[test]
    fn test_version() {
        shadow!(build);
        assert_eq!(build::PKG_VERSION, "0.1.0"); 
    }
}