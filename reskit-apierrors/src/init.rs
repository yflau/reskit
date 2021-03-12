use http_types::{StatusCode};

use crate::{PVLost, APIErrorClass};

lazy_static! {
    pub static ref DEFAULT_ERRORSPACE: String = String::from("");
    pub static ref BUILTIN_APP_NAME: String = String::from("");

	/// ERR_SUCCESS 请求成功
    /// 
    /// Mapping:
    /// - google api style guide: `google.rpc.Code.OK`
    /// - http status code: 200 OK
    ///
    /// Description:
	/// Not an error; returned on success
    pub static ref ERR_SUCCESS: APIErrorClass = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "0", 
        "Successful.", 
        StatusCode::Ok).with_pvlost(PVLost::Successful);

    /// ERR_UNKNOWN 未知的服务端错误，通常是服务端bug
    /// 
    /// Mapping:
    /// - google api style guide: `google.rpc.Code.UNKNOWN`
    /// - http status code: 500 Internal Server Error
    /// 
    /// Description:
    /// Unknown error.  For example, this error may be returned when
    /// a `Status` value received from another address space belongs to
    /// an error space that is not known in this address space.  Also
    /// errors raised by APIs that do not return enough error information
    /// may be converted to this error.
    pub static ref ERR_UNKNOWN: APIErrorClass = APIErrorClass::new(
        &*BUILTIN_APP_NAME, 
        "1", 
        "Unexpected error.", 
        StatusCode::InternalServerError);

    pub static ref BUILTIN_API_ERROR_CLASSES: Vec<&'static APIErrorClass> = vec![
        &*ERR_SUCCESS,
        &*ERR_UNKNOWN,
    ];
}

#[cfg(test)]
mod test {
    #[test]
    fn test_init() {
        use crate::{APIErrorMeta};
        use super::{BUILTIN_API_ERROR_CLASSES};
        let code = BUILTIN_API_ERROR_CLASSES.get(0).unwrap().code();
        assert_eq!(code, "0");
        assert_eq!(module_path!(), "reskit_apierrors::init::test");
    }
}
