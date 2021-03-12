use std::collections::HashMap;

use crate::{APIErrorMeta, APIErrorClass, BUILTIN_APP_NAME, BUILTIN_API_ERROR_CLASSES};

#[derive(Debug)]
pub struct Errorspace<'a> {
    errors: HashMap<String, HashMap<String, &'a APIErrorClass>>,
}

impl<'a> Errorspace<'a> {
    pub fn new() -> Errorspace<'a> {
        let mut space = Errorspace { errors: HashMap::new() };
        let system = space.errors.entry(String::from(&*BUILTIN_APP_NAME)).or_insert(HashMap::new());
        for class in &*BUILTIN_API_ERROR_CLASSES {
            system.entry(String::from(class.code())).or_insert(class);
        }
        space
    }

    pub fn register_api_error_class(&mut self, class: &'a APIErrorClass) {
        let system = self.errors.entry(String::from(class.system())).or_insert(HashMap::new());
        system.entry(String::from(class.code())).or_insert(class);
    }

    pub fn get_api_error_class(&self, system: &str, code: &str) -> Option<&APIErrorClass> {
        match self.errors.get(system) {
            Some(app) => app.get(code).copied(),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_errorspace() {
        use http_types::{StatusCode};
        use super::{Errorspace, APIErrorClass, APIErrorMeta};
        let mut space = Errorspace::new();
        let class = APIErrorClass::new("dummy", "1", "dummy error", StatusCode::InternalServerError);
        space.register_api_error_class(&class);
        assert_eq!(space.get_api_error_class("", "1").unwrap().code(), "1");
        assert_eq!(space.get_api_error_class("dummy", "1").unwrap().message(), "dummy error");
    }
}
