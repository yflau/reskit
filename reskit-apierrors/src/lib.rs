#[macro_use]
extern crate shadow_rs;

shadow!(build);

pub mod status_code;
pub mod apierror;
pub mod globals;
pub mod errorspace;
pub mod builtin;
pub mod macros;
pub mod prelude;

#[cfg(feature = "pvlost")]
pub mod pvlost;

#[allow(unused_qualifications)]
pub use apierror::{APIErrorMeta, APIErrorMetas, APIError};
pub use errorspace::Errorspace;
pub use builtin::Builtin;
pub use globals::{
    GLOBAL_ERRORSPACE_NAME,
    ERRORSPACES,
    new_errorspace,
    register_errorspace,
    clone_errorspace,
    register_api_error_metas,
    overwrite_api_error_metas,
    get_api_error_meta,
    register_api_error_metas_errorspace,
    overwrite_api_error_metas_errorspace,
    get_api_error_meta_errorspace,
};

#[cfg(feature = "pvlost")]
pub use pvlost::PVLost;

pub use reskit_apierrors_derive::*;

#[cfg(test)]
mod tests {
    #[test]
    fn test_version() {
        shadow!(build);
        assert_eq!(build::PKG_VERSION, "0.1.0"); 
    }
}