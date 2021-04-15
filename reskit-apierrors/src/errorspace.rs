use std::collections::HashMap;

use crate::{APIErrorMeta, APIError};

#[derive(Clone)]
pub struct Errorspace<'a> {
    errors: HashMap<&'a str, HashMap<&'a str, &'a dyn APIErrorMeta>>,
}

impl<'a> Errorspace<'a> {
    pub fn new() -> Errorspace<'a> {
        Errorspace { errors: HashMap::new() }
    }

    /// register_api_error_class register api error meta, if exists then ignore
    pub fn register_api_error_meta(&mut self, meta: &'a dyn APIErrorMeta) {
        let system = self.errors.entry(meta.system()).or_insert(HashMap::new());
        system.entry(meta.code()).or_insert(meta);
    }

    /// overwrite_api_error_class overwrite existing api error meta, used for stauts code rebinding
    pub fn overwrite_api_error_meta(&mut self, meta: &'a dyn APIErrorMeta) {
        let system = self.errors.entry(meta.system()).or_insert(HashMap::new());
        system.insert(meta.code(), meta);
    }

    pub fn get_api_error_meta(&self, system: &str, code: &str) -> Option<&'a dyn APIErrorMeta> {
        match self.errors.get(system) {
            Some(app) => app.get(code).copied(),
            None => None,
        }
    }

    pub fn len(&self, system: &str) -> usize {
        match self.errors.get(system) {
            Some(app) => app.len(),
            None => 0,
        }
    }

    /// adapt adapts anyhow::Error to specify error space, or wrap it with default_meta as a APIError
    pub(crate) fn adapt(&self, 
        err: anyhow::Error, 
        default_meta: &'static dyn APIErrorMeta, 
        mapping_names: &[&str],
        caller: Option<&'static str>,
    ) -> APIError<'a> {
        let api_err: APIError;
        if let Some(ae) = err.downcast_ref::<APIError>() {
            let meta = self.get_api_error_meta(ae.system(), ae.code());
            api_err = APIError {
                meta: meta.unwrap(),
                error: err,
                caller,
            }
        } else {
            // FIXME: do we need verbose gate here?
            api_err = APIError {
                meta: default_meta,
                error: err,
                caller,
            }
        }
        // TODO: map to other error spaces
        dbg!(mapping_names);
        api_err
    }

    /// force wraps the anyhow::Error with given meta as a APIError
    pub(crate) fn force(&self, 
        err: anyhow::Error, 
        meta: &'a dyn APIErrorMeta,
        mapping_names: &[&str],
        caller: Option<&'static str>,
    ) -> APIError<'a> {
        dbg!(mapping_names);
        APIError {
            meta,
            error: err,
            caller,
        }
    }

    /// map map the anyhow::Error with specified mappings
    fn _map(&self, err: anyhow::Error, mapping_names: &[&str]) -> anyhow::Error {
        dbg!("{}{}", err, mapping_names);
        anyhow::anyhow!("TODO")
    }
}

#[cfg(test)]
mod tests {
    use http_types::StatusCode;
    use reskit_utils::init_once;
    use anyhow::{anyhow, Result, Context};
    use crate::{ERRORSPACES, Builtin, adapt, force};
    use crate::apierror::APIErrorClass; // FIXME

    // FIXME: 完成apierrors_derive后修复此测试！
    // #[test]
    // fn test_errorspace() {
    //     init_once();
    //     let class: APIErrorClass = APIErrorClass::new("dummy", "1", "dummy error", StatusCode::InternalServerError);
    //     let mut spaces = ERRORSPACES.write().unwrap();
    //     let space = spaces.get_mut("").unwrap();
    //     space.register_api_error_meta(&class);
    //     assert_eq!(space.get_api_error_meta("", "1").unwrap().code(), "1");
    //     assert_eq!(space.get_api_error_meta("dummy", "1").unwrap().message(), "dummy error");
    //     assert!(matches!(space.get_api_error_meta("dummy", "1").unwrap().status_code(), StatusCode::InternalServerError));
    //     let rebind_class = APIErrorClass::new("dummy", "1", "dummy error", StatusCode::Ok);
    //     space.register_api_error_meta(&rebind_class);
    //     assert_eq!(space.get_api_error_meta("dummy", "1").unwrap().message(), "dummy error");
    //     assert!(matches!(space.get_api_error_meta("dummy", "1").unwrap().status_code(), StatusCode::InternalServerError));
    //     let rebind_class2 = APIErrorClass::new("dummy", "1", "dummy error", StatusCode::Ok);
    //     space.overwrite_api_error_meta(&rebind_class2);
    //     assert_eq!(space.get_api_error_meta("dummy", "1").unwrap().message(), "dummy error");
    //     assert!(matches!(space.get_api_error_meta("dummy", "1").unwrap().status_code(), StatusCode::Ok));   
    // }

    #[test]
    fn test_clone() {
        init_once();
        let mut spaces = ERRORSPACES.write().unwrap();
        let space = spaces.get_mut("").unwrap();
        let mut space_clone = space.clone();
        assert_eq!(space_clone.get_api_error_meta("", "1").unwrap().code(), "1");
        let class = APIErrorClass::new("dummy_clone", "1", "dummy error", StatusCode::InternalServerError);
        space_clone.register_api_error_meta(&class);
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

    fn demo() -> Result<()>{
        Err(anyhow!("demo error"))
    }

    #[test]
    fn test_adapt() {
        init_once();
        let result = demo()
            .context("first")
            .map_err(|e| adapt!(e, &Builtin::Unknown))
            .map_err(|e| adapt!(e, &Builtin::Internal));
        match result {
            Err(err)=>{
                assert_eq!(format!("{}", err.root_cause()), "demo error");
                assert_eq!(format!("{}", err), "500::1:Unexpected error.:reskit_apierrors::errorspace::tests::test_adapt::{{closure}}->500::1:Unexpected error.:reskit_apierrors::errorspace::tests::test_adapt::{{closure}}->first"); // NOTE: do not use display, use debug instead
                assert_eq!(format!("{:?}", err), "500::1:Unexpected error.:reskit_apierrors::errorspace::tests::test_adapt::{{closure}}->500::1:Unexpected error.:reskit_apierrors::errorspace::tests::test_adapt::{{closure}}->first\n\nCaused by:\n    demo error");
            },
            _ => {},
        }

        let result = demo()
            .context("pre")
            .map_err(|e| adapt!(e, &Builtin::Unknown))
            .context("post");
        match result {
            Err(err)=>{
                assert_eq!(format!("{}", err.root_cause()), "demo error");
                assert_eq!(format!("{}", err), "post"); // NOTE: do not use display, use debug instead
                assert_eq!(format!("{:?}", err), "post\n\nCaused by:\n    0: 500::1:Unexpected error.:reskit_apierrors::errorspace::tests::test_adapt::{{closure}}->pre\n    1: demo error");
            },
            _ => {},
        }
    }

    #[test]
    fn test_force() {
        init_once();
        let result = demo()
            .context("first")
            .map_err(|e| adapt!(e, &Builtin::Unknown))
            .context("second")
            .map_err(|e| force!(e, &Builtin::Internal));
        match result {
            Err(err)=>{
                assert_eq!(format!("{}", err.root_cause()), "demo error");
                assert_eq!(format!("{}", err), "500::2:Failure.:reskit_apierrors::errorspace::tests::test_force::{{closure}}->second"); // NOTE: do not use display, use debug instead
                assert_eq!(format!("{:?}", err), "500::2:Failure.:reskit_apierrors::errorspace::tests::test_force::{{closure}}->second\n\nCaused by:\n    0: 500::1:Unexpected error.:reskit_apierrors::errorspace::tests::test_force::{{closure}}->first\n    1: demo error");
            },
            _ => {},
        }
    }
}
