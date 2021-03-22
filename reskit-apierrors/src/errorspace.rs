use std::{fmt::{Display, Result, Formatter, Debug}};
use std::collections::HashMap;
use std::error::{Error};

use anyhow;
use http_types::{StatusCode};

use crate::{PVLost, APIErrorMeta, APIError, BUILTIN_APP_NAME};

//#[derive(Clone)] // FIXME
pub struct Errorspace {
    errors: HashMap<String, HashMap<String, Box<dyn APIErrorMeta>>>,
}

impl Errorspace {
    pub fn new() -> Errorspace {
        let mut space = Errorspace { errors: HashMap::new() };
        space.errors.entry(String::from(*BUILTIN_APP_NAME)).or_insert(HashMap::new());
        space
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

// impl<'a> Default for Errorspace<'a> {
//     fn default() -> Self {
//         DEFAULT_ERRORSPACE.read().unwrap()
//     }
// }

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
    use http_types::{StatusCode};
    use reskit_utils::{init_now};
    use crate::{APIErrorClass, DEFAULT_ERRORSPACE};
    
    #[test]
    fn test_errorspace() {
        init_now();
        //let mut space = Errorspace::default();
        let mut space = DEFAULT_ERRORSPACE.write().unwrap();
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

    // #[test]
    // fn test_clone() {
    //     //let space = Errorspace::default();
    //     let space = DEFAULT_ERRORSPACE.read().unwrap();
    //     let mut space_clone = space.clone();
    //     assert_eq!(space_clone.get_api_error_meta("", "1").unwrap().code(), "1");
    //     let class = APIErrorClass::new("dummy", "1", "dummy error", StatusCode::InternalServerError);
    //     space_clone.register_api_error_meta(&class);
    //     assert_eq!(space_clone.get_api_error_meta("dummy", "1").unwrap().message(), "dummy error");
    //     assert!(matches!(space_clone.get_api_error_meta("dummy", "1").unwrap().status_code(), StatusCode::InternalServerError));
    //     match space.get_api_error_meta("dummy", "1") {
    //         None =>{},
    //         Some(_class) => {
    //             assert!(true, "dummy:1 shoud None in default space");
    //         }
    //     }
    // }
}
