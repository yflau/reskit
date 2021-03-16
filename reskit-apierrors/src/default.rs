use crate::{APIErrorClass, APIErrorMeta, DEFAULT_ERRORSPACE};

/// register_api_error_class register api error meta, if exists then ignore
pub fn register_api_error_class(class: &'static APIErrorClass) {
    let mut space = DEFAULT_ERRORSPACE.write().unwrap();
    space.register_api_error_class(class);
}

/// overwrite_api_error_class overwrite existing api error meta, used for stauts code rebinding
pub fn overwrite_api_error_class(class: &'static APIErrorClass) {
    let mut space = DEFAULT_ERRORSPACE.write().unwrap();
    space.overwrite_api_error_class(class);
}

pub fn get_api_error_class(system: &str, code: &str) -> Option<APIErrorClass> {
    if let Some(class) = DEFAULT_ERRORSPACE.read().unwrap().get_api_error_class(system, code) {
        Some(APIErrorClass::new(
            class.system(), 
            class.code(), 
        class.message(), 
            class.status_code()).with_pvlost(class.pvlost()))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_default_errorspace() {
        use http_types::{StatusCode};
        use crate::{APIErrorMeta};
        use super::{get_api_error_class};
        assert_eq!(get_api_error_class("", "1").unwrap().message(), "Unexpected error.");
        assert!(matches!(get_api_error_class("", "1").unwrap().status_code(), StatusCode::InternalServerError));   
    }
}
