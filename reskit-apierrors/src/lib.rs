#[macro_use]
extern crate lazy_static;

pub mod pvlost;
pub mod apierror;
pub mod init;
pub mod errorspace;

pub use pvlost::{PVLost};
pub use apierror::{APIErrorMeta, APIErrorClass};
pub use init::{
    DEFAULT_ERRORSPACE, 
    BUILTIN_APP_NAME, 
    BUILTIN_API_ERROR_CLASSES,
    ERR_SUCCESS,
    ERR_UNKNOWN};
pub use errorspace::{Errorspace};
