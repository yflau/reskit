#[macro_use]
extern crate lazy_static;

pub mod pvlost;
pub mod apierror;
pub mod init;
pub mod errorspace;

pub use pvlost::{PVLost};
pub use apierror::{APIErrorMeta, APIErrorClass};
pub use init::{
    DEFAULT_ERRORSPACE_NAME, 
    BUILTIN_APP_NAME, 
    BUILTIN_API_ERROR_CLASSES,
    DEFAULT_ERRORSPACE,
    ERR_SUCCESS,
    ERR_UNKNOWN,
    ERR_INTERNAL,
    ERR_PARAMETERS,
    ERR_SIGNATURE,
    ERR_LICENSE_EXPIRED,
    ERR_NOT_IMPLEMENTED,
    ERR_NOT_FOUND,
    ERR_MULTI_FOUND,
    ERR_HTTP_BODY_EMPTY,
    ERR_XML_SYNTAX,
    ERR_REQUEST_METHOD,
    ERR_NO_LOGIN,
    ERR_PERMISSION_DENIED,
    ERR_STORAGE_FULL,
    ERR_DATA_SOURCE_FAILURE,
    ERR_TOO_HIGH_RATE,
    ERR_FAILED_PRECONDITION,
    ERR_OUT_OF_RANGE,
    ERR_ALREADY_EXISTS,
    ERR_ABORTED,
    ERR_CANCELLED,
    ERR_DEADLINE_EXCEEDED,
    ERR_UNAVAILABLE,
    ERR_DATA_LOSS,
};
pub use errorspace::{Errorspace};
