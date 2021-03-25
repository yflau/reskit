use std::fmt::{Display, Result, Formatter, Debug};
use std::collections::HashMap;
use std::error::Error;

use anyhow;
use http_types::StatusCode;

use crate::{PVLost, APIErrorMeta, APIError, CloneAPIErrorMeta};

#[derive(Clone)]
pub struct Errorspace {
    errors: HashMap<String, HashMap<String, Box<dyn APIErrorMeta>>>,  // FIXME: should we use static borrow?
}

impl Errorspace {
    pub fn new() -> Errorspace {
        Errorspace { errors: HashMap::new() }
    }

    /// register_api_error_class register api error meta, if exists then ignore
    pub fn register_api_error_meta(&mut self, meta: Box<dyn APIErrorMeta>) {
        let system = self.errors.entry(String::from(meta.system())).or_insert(HashMap::new());
        system.entry(String::from(meta.code())).or_insert(meta);
    }

    /// overwrite_api_error_class overwrite existing api error meta, used for stauts code rebinding
    pub fn overwrite_api_error_meta(&mut self, meta: Box<dyn APIErrorMeta>) {
        let system = self.errors.entry(String::from(meta.system())).or_insert(HashMap::new());
        system.insert(String::from(meta.code()), meta);
    }

    pub fn get_api_error_meta(&self, system: &str, code: &str) -> Option<&Box<dyn APIErrorMeta>> {
        match self.errors.get(system) {
            Some(app) => app.get(code),
            None => None,
        }
    }

    pub fn len(&self, system: &str) -> usize {
        match self.errors.get(system) {
            Some(app) => app.len(),
            None => 0,
        }
    }

    pub fn adapt(&self, err: anyhow::Error, default_meta: Box<dyn APIErrorMeta>, mapping_names: &[&str])
        -> impl APIError
    {
        self._adapt(3, err, default_meta, mapping_names)
    }

    fn _adapt(&self, _skip: usize, err: anyhow::Error, default_meta: Box<dyn APIErrorMeta>, _mapping_names: &[&str])
        -> impl APIError
    {
        WithDetail {
            meta: default_meta,
            error: err, // &*ERR_UNKNOWN,
            caller: None,
        }
    }
}

#[derive(Debug)]
struct WithDetail {
    meta: Box<dyn APIErrorMeta>,
    error: anyhow::Error,
    caller: Option<String>,
}

impl Display for WithDetail {
    fn fmt(&self, f: &mut Formatter) -> Result {
        std::fmt::Display::fmt(&self.meta, f) // FIXME: 需要结合meta和error！
    }
}

impl Error for WithDetail {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error.source()
    }
}

impl CloneAPIErrorMeta for WithDetail {
    fn clone_meta(&self) -> Box<dyn APIErrorMeta> {
        self.meta.clone()
    }
}

impl APIErrorMeta for WithDetail {
    fn system(&self) -> &str {
        &self.meta.system()
    }

    fn code(&self) -> &str {
        &self.meta.code()
    }

    fn message(&self) -> &str {
        &self.meta.message()
    }

    fn status_code(&self) -> StatusCode {
        self.meta.status_code()
    }

    fn pvlost(&self) -> PVLost {
        self.meta.pvlost()
    }
}

impl<'a> APIError for WithDetail{}

#[cfg(test)]
mod test {
    use http_types::StatusCode;
    use reskit_utils::init_once;
    use crate::ERRORS;
    use crate::apierror::APIErrorClass; // FIXME
   
    #[test]
    fn test_errorspace() {
        init_once();
        let mut space = ERRORS.write().unwrap();
        let class: APIErrorClass = APIErrorClass::new("dummy", "1", "dummy error", StatusCode::InternalServerError);
        space.register_api_error_meta(Box::new(class));
        assert_eq!(space.get_api_error_meta("", "1").unwrap().code(), "1");
        assert_eq!(space.get_api_error_meta("dummy", "1").unwrap().message(), "dummy error");
        assert!(matches!(space.get_api_error_meta("dummy", "1").unwrap().status_code(), StatusCode::InternalServerError));
        let rebind_class = APIErrorClass::new("dummy", "1", "dummy error", StatusCode::Ok);
        space.register_api_error_meta(Box::new(rebind_class));
        assert_eq!(space.get_api_error_meta("dummy", "1").unwrap().message(), "dummy error");
        assert!(matches!(space.get_api_error_meta("dummy", "1").unwrap().status_code(), StatusCode::InternalServerError));
        let rebind_class2 = APIErrorClass::new("dummy", "1", "dummy error", StatusCode::Ok);
        space.overwrite_api_error_meta(Box::new(rebind_class2));
        assert_eq!(space.get_api_error_meta("dummy", "1").unwrap().message(), "dummy error");
        assert!(matches!(space.get_api_error_meta("dummy", "1").unwrap().status_code(), StatusCode::Ok));   
    }

    #[test]
    fn test_clone() {
        init_once();
        let space = ERRORS.read().unwrap();
        let mut space_clone = space.clone();
        assert_eq!(space_clone.get_api_error_meta("", "1").unwrap().code(), "1");
        let class = APIErrorClass::new("dummy_clone", "1", "dummy error", StatusCode::InternalServerError);
        space_clone.register_api_error_meta(Box::new(class));
        assert_eq!(space.len("dummy_clone"), 0);
        assert_eq!(space_clone.len("dummy_clone"), 1);
        assert_eq!(space_clone.get_api_error_meta("dummy_clone", "1").unwrap().message(), "dummy error");
        assert!(matches!(space_clone.get_api_error_meta("dummy_clone", "1").unwrap().status_code(), StatusCode::InternalServerError));
        match space.get_api_error_meta("dummy_clone", "1") {
            None =>{},
            Some(_class) => {
                assert!(true, "dummy:1 shoud None in default space");
            }
        }
    }
}
