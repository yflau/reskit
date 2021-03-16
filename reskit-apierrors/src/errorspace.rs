use std::fmt::{Display, Result, Formatter};
use std::collections::HashMap;
use std::error::{Error};

use anyhow;
use http_types::{StatusCode};

use crate::{PVLost};
use crate::{APIErrorMeta, APIErrorClass, BUILTIN_APP_NAME, BUILTIN_API_ERROR_CLASSES};

#[derive(Debug, Clone)]
pub struct Errorspace<'a> {
    errors: HashMap<&'a str, HashMap<&'a str, &'a APIErrorClass>>,
}

impl<'a> Errorspace<'a> {
    pub fn new() -> Errorspace<'a> {
        let mut space = Errorspace { errors: HashMap::new() };
        space.errors.entry(*BUILTIN_APP_NAME).or_insert(HashMap::new());
        space
    }

    /// register_api_error_class register api error meta, if exists then ignore
    pub fn register_api_error_class(&mut self, class: &'a APIErrorClass) {
        let system = self.errors.entry(class.system()).or_insert(HashMap::new());
        system.entry(class.code()).or_insert(class);
    }

    /// overwrite_api_error_class overwrite existing api error meta, used for stauts code rebinding
    pub fn overwrite_api_error_class(&mut self, class: &'a APIErrorClass) {
        let system = self.errors.entry(class.system()).or_insert(HashMap::new());
        system.insert(class.code(), class);
    }

    pub fn get_api_error_class(&self, system: &str, code: &str) -> Option<&APIErrorClass> {
        match self.errors.get(system) {
            Some(app) => app.get(code).copied(),
            None => None,
        }
    }

    pub fn adapt(&self, err: anyhow::Error, default_class: &'a APIErrorClass, mapping_names: &[&str])
        -> impl 'a+Error + APIErrorMeta
    {
        self._adapt(3, err, default_class, mapping_names)
    }

    fn _adapt(&self, _skip: usize, err: anyhow::Error, default_class: &'a APIErrorClass, _mapping_names: &[&str])
        -> impl 'a+Error+APIErrorMeta
    {
        WithDetail {
            meta: default_class,
            error: err, // &*ERR_UNKNOWN,
            caller: None,
        }
    }
}

impl<'a> Default for Errorspace<'a> {
    fn default() -> Self {
        let mut space = Self::new();
        for class in &*BUILTIN_API_ERROR_CLASSES {
            space.register_api_error_class(class)
        }
        space
    }
}

#[derive(Debug)]
struct WithDetail<'a> {
    meta: &'a APIErrorClass,
    error: anyhow::Error,
    caller: Option<String>,
}

impl<'a> Display for WithDetail<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.meta.fmt(f) // TODO
    }
}

impl<'a> Error for WithDetail<'a> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error.source()
    }
}

impl<'a> APIErrorMeta for WithDetail<'a> {
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

#[cfg(test)]
mod test {
    use http_types::{StatusCode};
    use super::{Errorspace, APIErrorClass, APIErrorMeta};
    #[test]
    fn test_errorspace() {
        let mut space = Errorspace::default();
        let class = APIErrorClass::new("dummy", "1", "dummy error", StatusCode::InternalServerError);
        space.register_api_error_class(&class);
        assert_eq!(space.get_api_error_class("", "1").unwrap().code(), "1");
        assert_eq!(space.get_api_error_class("dummy", "1").unwrap().message(), "dummy error");
        assert!(matches!(space.get_api_error_class("dummy", "1").unwrap().status_code(), StatusCode::InternalServerError));
        let rebind_class =APIErrorClass::new("dummy", "1", "dummy error", StatusCode::Ok);
        space.register_api_error_class(&rebind_class);
        assert_eq!(space.get_api_error_class("dummy", "1").unwrap().message(), "dummy error");
        assert!(matches!(space.get_api_error_class("dummy", "1").unwrap().status_code(), StatusCode::InternalServerError));
        space.overwrite_api_error_class(&rebind_class);
        assert_eq!(space.get_api_error_class("dummy", "1").unwrap().message(), "dummy error");
        assert!(matches!(space.get_api_error_class("dummy", "1").unwrap().status_code(), StatusCode::Ok));   
    }

    #[test]
    fn test_clone() {
        let space = Errorspace::default();
        let mut space_clone = space.clone();
        assert_eq!(space_clone.get_api_error_class("", "1").unwrap().code(), "1");
        let class = APIErrorClass::new("dummy", "1", "dummy error", StatusCode::InternalServerError);
        space_clone.register_api_error_class(&class);
        assert_eq!(space_clone.get_api_error_class("dummy", "1").unwrap().message(), "dummy error");
        assert!(matches!(space_clone.get_api_error_class("dummy", "1").unwrap().status_code(), StatusCode::InternalServerError));
        match space.get_api_error_class("dummy", "1") {
            None =>{},
            Some(_class) => {
                assert!(true, "dummy:1 shoud None in default space");
            }
        }
    }
}
