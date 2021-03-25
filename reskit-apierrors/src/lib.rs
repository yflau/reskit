#[macro_use]
extern crate lazy_static;

pub mod pvlost;
pub mod apierror;
pub mod globals;
pub mod errorspace;
pub mod builtin;
pub mod macros;

pub use pvlost::{PVLost};
pub use apierror::{APIErrorMeta, APIError, APIErrorMetaEnum, APIErrorClass};
pub use globals::{
    DEFAULT_ERRORSPACE_NAME, 
    BUILTIN_APP_NAME,
    DEFAULT_ERRORSPACE,
    register_api_error_meta_enum,
    overwrite_api_error_meta_enum};
pub use errorspace::{Errorspace};
pub use builtin::{BuiltinAPIErrorMeta};
