use std::sync::RwLock;

use crate::{APIErrorMetaEnum, Errorspace};

lazy_static! {
    pub static ref DEFAULT_ERRORSPACE_NAME: &'static str = "";
    pub static ref BUILTIN_APP_NAME: &'static str = "";
    pub static ref DEFAULT_ERRORSPACE: RwLock<Errorspace> = RwLock::new(Errorspace::new());
}

/// register_api_error_meta_enum register APIErrorMetaEnum, if variant exists(system:code) then ignore
pub fn register_api_error_meta_enum<E>() 
    where E: APIErrorMetaEnum + 'static
{
    let mut space = DEFAULT_ERRORSPACE.write().unwrap();
    for meta in E::iter() {
        space.register_api_error_meta(Box::new(meta));
    }
}

/// overwrite_api_error_meta_enum overwrite existing api error meta with APIErrorMetaEnum, used for stauts code rebinding
pub fn overwrite_api_error_meta_enum<E>() 
    where E: APIErrorMetaEnum + 'static
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
    use http_types::{StatusCode};
    use reskit_utils::{INITS, init_now};
    use crate::{DEFAULT_ERRORSPACE};
        
    #[test]
    fn test_init() {
        init_now();
        assert_eq!(1, INITS.len());
        let space = DEFAULT_ERRORSPACE.read().unwrap();
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
