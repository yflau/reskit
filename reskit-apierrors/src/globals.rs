use std::sync::RwLock;

use lazy_static::lazy_static;

use crate::{APIErrorMetaEnum, Errorspace};

lazy_static! {
    /// ERRORS is the default `Errorspace`.
    pub static ref ERRORS: RwLock<Errorspace> = RwLock::new(Errorspace::new());

    //pub static ref ERRORSPACES: RwLock<HashMap<String, Errorspace>> = RwLock.new();
}

/// register_api_error_meta_enum register APIErrorMetaEnum, if variant exists(system:code) then ignore
pub fn register_api_error_meta_enum<E>() 
    where E: APIErrorMetaEnum + 'static
{
    let mut space = ERRORS.write().unwrap();
    for meta in E::iter() {
        space.register_api_error_meta(Box::new(meta));
    }
}

/// overwrite_api_error_meta_enum overwrite existing api error meta with APIErrorMetaEnum, used for stauts code rebinding
pub fn overwrite_api_error_meta_enum<E>() 
    where E: APIErrorMetaEnum + 'static
{
    let mut space = ERRORS.write().unwrap();
    for meta in E::iter() {
        space.overwrite_api_error_meta(Box::new(meta));
    }
}

/// FXIME: how to ref? Or do not use this?
// pub fn get_api_error_class(system: &str, code: &str) -> Option<&'static Box<dyn APIErrorMeta>> {
//     ERRORS.read().unwrap().get_api_error_meta(system, code)
// }

#[cfg(test)]
mod tests {
    use http_types::StatusCode;
    use reskit_utils::init_once;
    use crate::ERRORS;
        
    #[test]
    fn test_init() {
        init_once();
        let space = ERRORS.read().unwrap();
        let err = space.get_api_error_meta("", "2").unwrap();
        assert_eq!(err.status_code(), StatusCode::InternalServerError);
        assert_eq!(err.code(), "2");
        assert_eq!(err.system(), "");
        assert_eq!(err.message(), "Failure.");
    }

    // #[test]
    // fn test_default_errorspace() {
    //     use http_types::{StatusCode};
    //     use crate::{APIErrorMeta};
    //     use super::{get_api_error_class};
    //     assert_eq!(get_api_error_class("", "1").unwrap().message(), "Unexpected error.");
    //     assert!(matches!(get_api_error_class("", "1").unwrap().status_code(), StatusCode::InternalServerError));   
    // }
}
