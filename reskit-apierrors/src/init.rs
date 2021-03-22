use std::sync::RwLock;

use crate::{Errorspace};

lazy_static! {
    pub static ref DEFAULT_ERRORSPACE_NAME: &'static str = "";
    pub static ref BUILTIN_APP_NAME: &'static str = "";
    pub static ref DEFAULT_ERRORSPACE: RwLock<Errorspace> = RwLock::new(Errorspace::new());
    //pub static ref INIT: () = init_now();
}

#[cfg(test)]
mod test {
    use http_types::{StatusCode};
    use reskit_utils::{INITS, init_now};
    use crate::{DEFAULT_ERRORSPACE};
    #[test]
    fn test_init() {
        init_now();
        assert_eq!(1, INITS.len());
        let space = DEFAULT_ERRORSPACE.read().unwrap();
        let err = space.get_api_error_meta("", "2").unwrap();
        assert_eq!(err.status_code(), StatusCode::InternalServerError);
        assert_eq!(err.code(), "2");
        assert_eq!(err.system(), "");
        assert_eq!(err.message(), "Failure.");
    }
}
