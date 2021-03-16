use http_types::{StatusCode};

use crate::{APIErrorMeta, APIErrorClass};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum BuiltinAPIErrorMeta {
    // #[apierror(system="", code="1", message="Successful.", status_code=200, pvlost=0)]
    Successful,
    Unknown,
    Internal,
    // ...
}

impl BuiltinAPIErrorMeta {
    pub fn meta(&self) -> impl APIErrorMeta{
        APIErrorClass::new("", "1", "test", StatusCode::Ok) // FIXME
    }
}

#[cfg(test)]
mod test {
    use super::{BuiltinAPIErrorMeta, APIErrorMeta};
    #[test]
    fn test_builtin() {
        assert_eq!(BuiltinAPIErrorMeta::Unknown.meta().message(), "test");
    }
}
