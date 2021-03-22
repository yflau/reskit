use std::sync::RwLock;

use crate::{Errorspace};

use crate::builtin::{init, INITS};

lazy_static! {
    pub static ref DEFAULT_ERRORSPACE_NAME: &'static str = "";
    pub static ref BUILTIN_APP_NAME: &'static str = "";
    pub static ref DEFAULT_ERRORSPACE: RwLock<Errorspace> = create_default_errorspace();
    // pub static ref DEFAULT_ERRORSPACE: RwLock<Errorspace> = RwLock::new(Errorspace::new());
    // pub static ref INIT: () = init();
}

fn create_default_errorspace() -> RwLock<Errorspace> {
    for f in INITS {
        f()
    }
    RwLock::new(Errorspace::new())
}

#[cfg(test)]
mod test {
    use http_types::{StatusCode};
    use crate::{DEFAULT_ERRORSPACE};
    use crate::builtin::{INITS};
    //use super::{INIT};
    #[test]
    fn test_init() {
        //*INIT;
        assert_eq!(1, INITS.len());
        let space = DEFAULT_ERRORSPACE.read().unwrap();
        let err = space.get_api_error_meta("", "2").unwrap();
        assert_eq!(err.status_code(), StatusCode::InternalServerError);
        assert_eq!(err.code(), "2");
        assert_eq!(err.system(), "");
        assert_eq!(err.message(), "Failure.");
    }
}
