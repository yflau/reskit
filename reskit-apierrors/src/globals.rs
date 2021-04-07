use std::sync::RwLock;

use lazy_static::lazy_static;

use crate::{Errorspace, APIErrorMeta, APIErrorMetas};

lazy_static! {
    /// ERRORS is the default `Errorspace`.
    pub static ref ERRORS: RwLock<Errorspace<'static>> = RwLock::new(Errorspace::new());

    //pub static ref ERRORSPACES: RwLock<HashMap<String, Errorspace>> = RwLock.new();
}

/// register_api_error_meta_enum register APIErrorMetaEnum, if variant exists(system:code) then ignore
pub fn register_api_error_meta_enum<E>() where E: APIErrorMetas + 'static {
    let mut space = ERRORS.write().unwrap();
    for meta in E::api_error_metas() {
        space.register_api_error_meta(meta);
    }
}

/// overwrite_api_error_meta_enum overwrite existing api error meta with APIErrorMetaEnum, used for stauts code rebinding
pub fn overwrite_api_error_meta_enum<E>() where E: APIErrorMetas + 'static {
    let mut space = ERRORS.write().unwrap();
    for meta in E::api_error_metas() {
        space.overwrite_api_error_meta(meta);
    }
}

/// get_api_error_meta get api error meta for specified systen & code
pub fn get_api_error_meta(system: &str, code: &str) -> Option<&'static dyn APIErrorMeta> {
    ERRORS.read().unwrap().get_api_error_meta(system, code)
}

/// adapt adapts anyhow::Error to specify error space, or wrap it with default_meta as a APIError in global error space
pub fn adapt(err: anyhow::Error, default_meta: &'static dyn APIErrorMeta, _mapping_names: &[&str]) -> anyhow::Error {
    let api_err = ERRORS.read().unwrap().adapt(err, default_meta, _mapping_names);
    anyhow::Error::new(api_err)
}

/// force wraps the anyhow::Error with given meta as a APIError in global error space
pub fn force(err: anyhow::Error, default_meta: &'static dyn APIErrorMeta, _mapping_names: &[&str]) -> anyhow::Error {
    let api_err = ERRORS.read().unwrap().force(err, default_meta, _mapping_names);
    anyhow::Error::new(api_err)
}

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

    #[test]
    fn test_default_errorspace() {
        init_once();
        use http_types::{StatusCode};
        use super::get_api_error_meta;
        assert_eq!(get_api_error_meta("", "1").unwrap().message(), "Unexpected error.");
        assert!(matches!(get_api_error_meta("", "1").unwrap().status_code(), StatusCode::InternalServerError));   
    }
}
