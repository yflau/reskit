use std::collections::HashMap;

use anyhow;

use crate::{APIErrorMeta, APIError};

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

    pub fn adapt<'a>(&'a self, err: anyhow::Error, default_meta: &'a Box<dyn APIErrorMeta>, _mapping_names: &[&str])
        -> APIError
    {
        let api_err: APIError;
        if let Some(ae) = err.downcast_ref::<APIError>() {
            let meta = self.get_api_error_meta(ae.system(), ae.code());
            api_err = APIError {
                meta: meta.unwrap(),
                error: err,
                meta_data: None,
            }
        } else {
            api_err = APIError {
                meta: &Box::new(default_meta),
                error: err,
                meta_data: None,
            }
        }
        api_err
    }

    pub fn force<'a>(&'a self, err: anyhow::Error, meta: &'a Box<dyn APIErrorMeta>, _mapping_names: &[&str])
        -> APIError
    {
        APIError {
            meta,
            error: err,
            meta_data: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use http_types::StatusCode;
    use reskit_utils::init_once;
    use anyhow::{anyhow, Result};
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

    #[test]
    fn test_adapt() {
        init_once();
        fn demo() -> Result<()>{
            Err(anyhow!("demo error"))
        }
        let space = ERRORS.read().unwrap();
        let default_meta = space.get_api_error_meta("", "1").unwrap();
        let result = demo().map_err(|e| space.adapt(e, default_meta, &[]));
        match result {
            Err(err)=>{
                //assert_eq!(format!("{}", err.root_cause()), "demo error");
                assert_eq!(format!("{}", err), "500::1:Unexpected error.:2"); // FIXME: 需要类似Debug的调用链表示
                assert_eq!(format!("{:?}", err), "APIError { meta: Unknown, error: demo error, meta_data: None }");
            },
            _ => {},
        }
    }

    #[test]
    fn test_force() {
        
    }
}
