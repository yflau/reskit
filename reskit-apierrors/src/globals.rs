use std::sync::RwLock;
use std::collections::HashMap;

use lazy_static::lazy_static;
use linkme::distributed_slice;
use reskit_utils::INITS;
use anyhow::Result;

use crate::{Errorspace, APIErrorMeta, APIErrorMetas, Builtin};

const GLOBAL_ERRORSPACE_NAME: &str = "";

lazy_static! {
    pub static ref ERRORSPACES: RwLock<HashMap<&'static str, Errorspace<'static>>> = RwLock::new(HashMap::new());
}

#[distributed_slice(INITS)]
pub(crate) fn init() {
    new_errorspace(GLOBAL_ERRORSPACE_NAME);
    register_api_error_metas::<Builtin>();
}

/// new_errorspace create new errorspace with name and register it
pub fn new_errorspace(name: &'static str) {
    ERRORSPACES.write().unwrap().entry(name).or_insert(Errorspace::new());
}

/// register_errorspace register errorspace
pub fn register_errorspace(name: &'static str, space: Errorspace<'static>) {
    ERRORSPACES.write().unwrap().entry(name).or_insert(space);
}

/// clone_errorspace clone errorspace
pub fn clone_errorspace(from: &'static str, to: &'static str) -> Result<()>{
    let mut spaces = ERRORSPACES.write().unwrap();
    match spaces.get(from) {
        None => Err(anyhow::anyhow!("errorspace {} not found", from)),
        Some(space) => {
            let space_clone = space.clone();
            spaces.entry(to).or_insert(space_clone);
            Ok(())
        }
    }
}

/// register_api_error_metas register APIErrorMetas, if variant exists(system:code) then ignore
pub fn register_api_error_metas_errorspace<T>(name: &str) where T: APIErrorMetas + 'static {
    let mut spaces = ERRORSPACES.write().unwrap();
    let space = spaces.get_mut(name).unwrap();
    for meta in T::api_error_metas() {
        space.register_api_error_meta(meta);
    }
}

/// register_api_error_metas register APIErrorMetas, if variant exists(system:code) then ignore
pub fn register_api_error_metas<T>() where T: APIErrorMetas + 'static {
    register_api_error_metas_errorspace::<T>(GLOBAL_ERRORSPACE_NAME);
}

/// overwrite_api_error_metas overwrite existing api error meta with APIErrorMetas, used for stauts code rebinding
pub fn overwrite_api_error_metas_errorspace<T>(name: &str) where T: APIErrorMetas + 'static {
    let mut spaces = ERRORSPACES.write().unwrap();
    let space = spaces.get_mut(name).unwrap();
    for meta in T::api_error_metas() {
        space.overwrite_api_error_meta(meta);
    }
}

/// overwrite_api_error_metas overwrite existing api error meta with APIErrorMetas, used for stauts code rebinding
pub fn overwrite_api_error_metas<T>() where T: APIErrorMetas + 'static {
    overwrite_api_error_metas_errorspace::<T>(GLOBAL_ERRORSPACE_NAME);
}

/// get_api_error_meta get api error meta for specified systen & code
pub fn get_api_error_meta_errorspace(name: &str, system: &str, code: &str) -> Option<&'static dyn APIErrorMeta> {
    ERRORSPACES.read().unwrap().get(name).unwrap().get_api_error_meta(system, code)
}

/// get_api_error_meta get api error meta for specified systen & code
pub fn get_api_error_meta(system: &str, code: &str) -> Option<&'static dyn APIErrorMeta> {
    get_api_error_meta_errorspace(GLOBAL_ERRORSPACE_NAME, system, code)
}

/// adapt_errorspace adapts anyhow::Error to specify error space, or wrap it with default_meta as a APIError in global error space
pub fn adapt_errorspace(name: &str, err: anyhow::Error, default_meta: &'static dyn APIErrorMeta, _mapping_names: &[&str]) -> anyhow::Error {
    let spaces = ERRORSPACES.read().unwrap();
    let space = spaces.get(name).unwrap();
    let api_err = space.adapt(err, default_meta, _mapping_names);
    anyhow::Error::new(api_err)
}

pub fn adapt(err: anyhow::Error, default_meta: &'static dyn APIErrorMeta, _mapping_names: &[&str]) -> anyhow::Error {
    adapt_errorspace(GLOBAL_ERRORSPACE_NAME, err, default_meta, _mapping_names)
}

/// force wraps the anyhow::Error with given meta as a APIError in global error space
pub fn force_errorspace(name: &str, err: anyhow::Error, meta: &'static dyn APIErrorMeta, _mapping_names: &[&str]) -> anyhow::Error {
    let spaces = ERRORSPACES.read().unwrap();
    let space = spaces.get(name).unwrap();
    let api_err = space.force(err, meta, _mapping_names);
    anyhow::Error::new(api_err)
}

pub fn force(err: anyhow::Error, meta: &'static dyn APIErrorMeta, _mapping_names: &[&str]) -> anyhow::Error {
    force_errorspace(GLOBAL_ERRORSPACE_NAME, err, meta, _mapping_names)
}

#[cfg(test)]
mod tests {
    use http_types::StatusCode;
    use reskit_utils::init_once;
    use super::{get_api_error_meta, get_api_error_meta_errorspace, clone_errorspace};
        
    #[test]
    fn test_init() {
        init_once();
        let err = get_api_error_meta("", "2").unwrap();
        assert_eq!(err.status_code(), StatusCode::InternalServerError);
        assert_eq!(err.code(), "2");
        assert_eq!(err.system(), "");
        assert_eq!(err.message(), "Failure.");
        assert_eq!(get_api_error_meta("", "1").unwrap().message(), "Unexpected error.");
        assert!(matches!(get_api_error_meta("", "1").unwrap().status_code(), StatusCode::InternalServerError)); 
    }

    #[test]
    fn test_clone_errorspaces() {
        init_once();
        clone_errorspace("", "clone").unwrap();
        let err = get_api_error_meta_errorspace("clone", "", "2").unwrap();
        assert_eq!(err.status_code(), StatusCode::InternalServerError);
        assert_eq!(err.code(), "2");
        assert_eq!(err.system(), "");
        assert_eq!(err.message(), "Failure.");
    }
}
