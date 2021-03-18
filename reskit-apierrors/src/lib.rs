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
pub use apierror::{APIErrorMeta, APIErrorClass};
pub use init::{
    DEFAULT_ERRORSPACE_NAME, 
    BUILTIN_APP_NAME,
    DEFAULT_ERRORSPACE};
pub use errorspace::{Errorspace};
pub use default::{
    register_api_error_class,
    overwrite_api_error_class,
    get_api_error_class,
};
