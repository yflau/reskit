#[macro_use]
extern crate shadow_rs;

shadow!(build);

pub mod apierror;
pub mod globals;
pub mod errorspace;
pub mod builtin;
pub mod macros;

#[cfg(feature = "pvlost")]
pub mod pvlost;

pub use apierror::{APIErrorMeta, APIErrorMetas, APIError, APIErrorMetaEnum};
pub use errorspace::Errorspace;
pub use builtin::Builtin;
pub use globals::{
    GLOBAL_ERRORSPACE_NAME,
    ERRORSPACES,
    new_errorspace,
    register_errorspace,
    clone_errorspace,
    register_api_error_metas,
    overwrite_api_error_metas
};

#[cfg(feature = "pvlost")]
pub use pvlost::PVLost;

#[cfg(test)]
mod tests {
    #[test]
    fn test_version() {
        shadow!(build);
        assert_eq!(build::PKG_VERSION, "0.1.0"); 
    }
}