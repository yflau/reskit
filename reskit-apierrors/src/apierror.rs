use std::fmt::{Display, Result, Formatter, Debug};

use http_types::{StatusCode};

use crate::{PVLost};

pub trait APIErrorMeta: Sync + Send + Debug + Display { // FIXME: add 'static?
    fn system(&self) -> &str;
    fn code(&self) -> &str;
    fn message(&self) -> &str;
    fn status_code(&self) -> StatusCode;
    fn pvlost(&self) -> PVLost;
}

#[derive(Debug, PartialEq)]
pub struct APIErrorClass {
    system: String,
    code: String,
    message: String, 
    status: StatusCode, // FIXME: use u16 instead for missing some non-standardcodes, e.g. 499 ?
    pvlost: PVLost,
}

impl Display for APIErrorClass {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}:{}:{}:{}:{}", self.status, self.system, self.code, self.message, self.pvlost as u8)
    }
}

impl APIErrorClass {
    pub fn new(system: &str, code: &str, msg: &str, status: StatusCode) -> APIErrorClass {
        APIErrorClass{
            system: system.to_string(),
            code: code.to_string(),
            message: msg.to_string(),
            status: status,
            pvlost: PVLost::LocalError,
        }
    }

    pub fn set_pvlost(&mut self, pvlost: PVLost) {
        self.pvlost = pvlost;
    }

    pub fn with_pvlost(mut self, pvlost: PVLost) -> APIErrorClass {
        self.pvlost = pvlost;
        self
    }
}

impl APIErrorMeta for APIErrorClass {
    fn system(&self) -> &str {
        &self.system
    }

    fn code(&self) -> &str {
        &self.code
    }

    fn message(&self) -> &str {
        &self.message
    }

    fn status_code(&self) -> StatusCode {
        self.status
    }

    fn pvlost(&self) -> PVLost {
        self.pvlost
    }
}

#[cfg(test)]
mod test {
    use http_types::{StatusCode};
    use crate::{APIErrorMeta, APIErrorClass, PVLost};

    #[test]
    fn test_api_error_class() {
        let mut dummy_err = APIErrorClass::new("test", "1", "dummy error", StatusCode::InternalServerError);
        assert_eq!(dummy_err.system(), "test");
        assert_eq!(dummy_err.code(), "1");
        assert_eq!(dummy_err.message(), "dummy error");
        assert!(matches!(dummy_err.status_code(), StatusCode::InternalServerError));
        assert_eq!(format!("{}", dummy_err), "500:test:1:dummy error:2");
        assert!(matches!(dummy_err.pvlost(), PVLost::LocalError));
        dummy_err.set_pvlost(PVLost::RemoteError);
        assert!(matches!(dummy_err.pvlost(), PVLost::RemoteError));
        assert_eq!(format!("{}", dummy_err), "500:test:1:dummy error:1");
        let xxx_err = APIErrorClass::new("xxx", "2", "xxx error", StatusCode::InternalServerError).with_pvlost(PVLost::RemoteError);
        assert!(matches!(xxx_err.pvlost(), PVLost::RemoteError));
    }

    #[test]
    fn test_class_equals() {
        let code = APIErrorClass::new(
            "test",
            "1",
            "test error",
            StatusCode::Ok).with_pvlost(PVLost::RemoteError);
        let eq1 = APIErrorClass::new(
            "test",
            "1",
            "test error",
            StatusCode::Ok).with_pvlost(PVLost::RemoteError);
        assert_eq!(code, eq1);
        let neq1 = APIErrorClass::new(
            "xxx",
            "1",
            "test error",
            StatusCode::Ok).with_pvlost(PVLost::RemoteError);
        assert_ne!(code, neq1);
        let neq2 = APIErrorClass::new(
            "test",
            "3",
            "test error",
            StatusCode::Ok).with_pvlost(PVLost::RemoteError);
        assert_ne!(code, neq2);
        let neq3 = APIErrorClass::new(
            "test",
            "1",
            "test error 2",
            StatusCode::Ok).with_pvlost(PVLost::RemoteError);
        assert_ne!(code, neq3);
        let neq4 = APIErrorClass::new(
            "test",
            "1",
            "test error",
            StatusCode::Ok).with_pvlost(PVLost::LocalError);
        assert_ne!(code, neq4);
    }
}
