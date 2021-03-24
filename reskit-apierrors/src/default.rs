use strum::{IntoEnumIterator};

use crate::{APIErrorMeta, DEFAULT_ERRORSPACE};

/// register_api_error_class register api error meta, if exists then ignore
pub fn register_api_error_metas<E>() 
    where E: IntoEnumIterator + APIErrorMeta + 'static
{
    let mut space = DEFAULT_ERRORSPACE.write().unwrap();
    for meta in E::iter() {
        space.register_api_error_meta(Box::new(meta));
    }
}

/// overwrite_api_error_class overwrite existing api error meta, used for stauts code rebinding
pub fn overwrite_api_error_metas<E>() 
    where E: IntoEnumIterator + APIErrorMeta + 'static
{
    let mut space = DEFAULT_ERRORSPACE.write().unwrap();
    for meta in E::iter() {
        space.overwrite_api_error_meta(Box::new(meta));
    }
}

/// FXIME: how to ref? Or do not use this?
// pub fn get_api_error_class(system: &str, code: &str) -> Option<&'static Box<dyn APIErrorMeta>> {
//     DEFAULT_ERRORSPACE.read().unwrap().get_api_error_meta(system, code)
// }

#[cfg(test)]
mod test {
    // #[test]
    // fn test_default_errorspace() {
    //     use http_types::{StatusCode};
    //     use crate::{APIErrorMeta};
    //     use super::{get_api_error_class};
    //     assert_eq!(get_api_error_class("", "1").unwrap().message(), "Unexpected error.");
    //     assert!(matches!(get_api_error_class("", "1").unwrap().status_code(), StatusCode::InternalServerError));   
    // }
}
