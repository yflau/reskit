use std::sync::RwLock;

use crate::{Errorspace};

lazy_static! {
    pub static ref DEFAULT_ERRORSPACE_NAME: &'static str = "";
    pub static ref BUILTIN_APP_NAME: &'static str = "";
    pub static ref DEFAULT_ERRORSPACE: RwLock<Errorspace<'static>> = RwLock::new(Errorspace::new());
}

#[cfg(test)]
mod test {
    use http_types::{StatusCode};
    use crate::{APIErrorMeta, DEFAULT_ERRORSPACE};
    #[test]
    fn test_init() {
        let space = DEFAULT_ERRORSPACE.read().unwrap();
        let err = space.get_api_error_meta("", "2").unwrap();
        assert_eq!(err.status_code(), StatusCode::InternalServerError);
        assert_eq!(err.code(), "2");
        assert_eq!(err.system(), "");
        assert_eq!(err.message(), "Failure.");
    }
}
