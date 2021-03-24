#[macro_use]
extern crate lazy_static;

pub mod pvlost;
pub mod apierror;
pub mod init;
pub mod errorspace;
pub mod default;
pub mod builtin;
pub mod macros;

pub use pvlost::{PVLost};
pub use apierror::{APIErrorMeta, APIError, APIErrorMetaEnum, APIErrorClass};
pub use init::{
    DEFAULT_ERRORSPACE_NAME, 
    BUILTIN_APP_NAME,
    DEFAULT_ERRORSPACE};
pub use errorspace::{Errorspace};
pub use default::{
    register_api_error_metas,
    overwrite_api_error_metas,
};
pub use builtin::{BuiltinAPIErrorMeta};
