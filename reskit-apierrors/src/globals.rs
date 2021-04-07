use std::sync::RwLock;
use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::{Errorspace, APIErrorMeta, APIErrorMetas};

lazy_static! {
    /// ERRORSPACE is the global `Errorspace`.
    pub static ref ERRORSPACE: RwLock<Errorspace<'static>> = RwLock::new(Errorspace::new());

    /// ERRORSPACES is registry for all errorspaces except global errorspace
    pub static ref ERRORSPACES: RwLock<HashMap<&'static str, &'static Errorspace<'static>>> = RwLock::new(HashMap::new());
}

/// register_errorspace register errorspace
pub fn register_errorspace(name: &'static str, space: &'static Errorspace<'static>) {
    ERRORSPACES.write().unwrap().entry(name).or_insert(space);
}

pub fn get_errorspace(name: &'static str) -> Option<&'static Errorspace<'static>>{
    ERRORSPACES.read().unwrap().get(name).copied()
}

/// register_api_error_metas register APIErrorMetas, if variant exists(system:code) then ignore
pub fn register_api_error_metas<E>() where E: APIErrorMetas + 'static {
    let mut space = ERRORSPACE.write().unwrap();
    for meta in E::api_error_metas() {
        space.register_api_error_meta(meta);
    }
}

/// overwrite_api_error_metas overwrite existing api error meta with APIErrorMetas, used for stauts code rebinding
pub fn overwrite_api_error_metas<E>() where E: APIErrorMetas + 'static {
    let mut space = ERRORSPACE.write().unwrap();
    for meta in E::api_error_metas() {
        space.overwrite_api_error_meta(meta);
    }
}

/// get_api_error_meta get api error meta for specified systen & code
pub fn get_api_error_meta(system: &str, code: &str) -> Option<&'static dyn APIErrorMeta> {
    ERRORSPACE.read().unwrap().get_api_error_meta(system, code)
}

/// adapt adapts anyhow::Error to specify error space, or wrap it with default_meta as a APIError in global error space
pub fn adapt(err: anyhow::Error, default_meta: &'static dyn APIErrorMeta, _mapping_names: &[&str]) -> anyhow::Error {
    let api_err = ERRORSPACE.read().unwrap().adapt(err, default_meta, _mapping_names);
    anyhow::Error::new(api_err)
}

/// force wraps the anyhow::Error with given meta as a APIError in global error space
pub fn force(err: anyhow::Error, meta: &'static dyn APIErrorMeta, _mapping_names: &[&str]) -> anyhow::Error {
    let api_err = ERRORSPACE.read().unwrap().force(err, meta, _mapping_names);
    anyhow::Error::new(api_err)
}

pub fn adapt_errorspace(
    space: &str, 
    err: anyhow::Error, 
    default_meta: &'static dyn APIErrorMeta, 
    _mapping_names: &[&str]) -> anyhow::Error {
        anyhow::Error::msg("todo")
}

pub fn force_errorspace(
    space: &str, 
    err: anyhow::Error, 
    default_meta: &'static dyn APIErrorMeta, 
    _mapping_names: &[&str]) -> anyhow::Error {
        anyhow::Error::msg("todo")
}

#[cfg(test)]
mod tests {
    use http_types::StatusCode;
    use reskit_utils::init_once;
    use crate::ERRORSPACE;
        
    #[test]
    fn test_init() {
        init_once();
        let space = ERRORSPACE.read().unwrap();
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
