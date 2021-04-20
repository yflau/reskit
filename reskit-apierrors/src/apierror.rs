use std::fmt::{Display, Result, Formatter, Debug};
use std::error::Error;

use http_types::StatusCode;
use strum::IntoEnumIterator;

#[cfg(feature = "pvlost")]
use crate::PVLost;

pub trait APIErrorMeta: Sync + Send + Debug + Display {
    fn system(&self) -> &str;
    fn code(&self) -> &str;
    fn message(&self) -> &str;
    fn status_code(&self) -> StatusCode;

    #[cfg(feature = "pvlost")]
    fn pvlost(&self) -> PVLost;
}

pub trait APIErrorMetas {
    fn api_error_metas() -> Vec<&'static dyn APIErrorMeta>;
}

pub trait APIErrorMetaEnum: IntoEnumIterator + APIErrorMeta{} // FIXME: do we need this?

#[derive(Debug)]
pub struct APIError<'a> {
    pub meta: &'a dyn APIErrorMeta, // TODO: static dispatch with enum_dispatch!
    pub error: anyhow::Error,
    pub caller: Option<&'static str>,

    // #[serde(skip_serializing_if = "extensions_is_empty")]
    // pub extensions: Option<Map<String, Value>>, // TODO: add extension support
}

impl<'a> Display for APIError<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.caller{
            Some(caller) => write!(f, "{}:{}->{}", self.meta, caller, self.error),
            None => write!(f, "{}->{}", self.meta, self.error),
        }
    }
}

impl<'a>  Error for APIError<'a> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error.source()
    }
}

impl<'a>  APIErrorMeta for APIError<'a>  {
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

    #[cfg(feature = "pvlost")]
    fn pvlost(&self) -> PVLost {
        self.meta.pvlost()
    }
}

/// APIErrorClass is a APIErrorMeta implementation used for single meta registration, you will not use this usually.
/// Deprecated, define `APIErrorMetaEnum` instead
#[derive(Clone, Debug, PartialEq)]
pub struct APIErrorClass {
    system: String,
    code: String,
    message: String, 
    status: StatusCode, // FIXME: use u16 instead for missing some non-standardcodes, e.g. 499 ?

    #[cfg(feature = "pvlost")]
    pvlost: PVLost,
}

#[cfg(not(feature = "pvlost"))]
impl Display for APIErrorClass {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}:{}:{}:{}", self.status, self.system, self.code, self.message)
    }
}

#[cfg(feature = "pvlost")]
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
            status,

            #[cfg(feature = "pvlost")]
            pvlost: PVLost::LocalError,
        }
    }

    #[cfg(feature = "pvlost")]
    pub fn set_pvlost(&mut self, pvlost: PVLost) {
        self.pvlost = pvlost;
    }

    #[cfg(feature = "pvlost")]
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

    #[cfg(feature = "pvlost")]
    fn pvlost(&self) -> PVLost {
        self.pvlost
    }
}

#[cfg(test)]
mod tests {
    use http_types::StatusCode;
    use crate::APIErrorMeta;
    use super::APIErrorClass;

    #[cfg(not(feature = "pvlost"))]
    #[test]
    fn test_api_error_class() {
        let dummy_err = APIErrorClass::new("test", "1", "dummy error", StatusCode::InternalServerError);
        assert_eq!(dummy_err.system(), "test");
        assert_eq!(dummy_err.code(), "1");
        assert_eq!(dummy_err.message(), "dummy error");
        assert!(matches!(dummy_err.status_code(), StatusCode::InternalServerError));
        assert_eq!(format!("{}", dummy_err), "500:test:1:dummy error");
    }

    #[cfg(feature = "pvlost")]
    #[test]
    fn test_api_error_class() {
        use crate::PVLost;
        let mut dummy_err = APIErrorClass::new("test", "1", "dummy error", StatusCode::InternalServerError);
        assert_eq!(dummy_err.system(), "test");
        assert_eq!(dummy_err.code(), "1");
        assert_eq!(dummy_err.message(), "dummy error");
        assert!(matches!(dummy_err.status_code(), StatusCode::InternalServerError));
        assert_eq!(format!("{}", dummy_err), "500:test:1:dummy error");
        assert_eq!(format!("{}", dummy_err), "500:test:1:dummy error:2");
        assert!(matches!(dummy_err.pvlost(), PVLost::LocalError));
        dummy_err.set_pvlost(PVLost::RemoteError);
        assert!(matches!(dummy_err.pvlost(), PVLost::RemoteError));
        assert_eq!(format!("{}", dummy_err), "500:test:1:dummy error:1");
        let xxx_err = APIErrorClass::new("xxx", "2", "xxx error", StatusCode::InternalServerError).with_pvlost(PVLost::RemoteError);
        assert!(matches!(xxx_err.pvlost(), PVLost::RemoteError));
    }
}
