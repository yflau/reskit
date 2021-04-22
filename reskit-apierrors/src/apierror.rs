use std::fmt::{Display, Result, Formatter, Debug};
use std::error::Error;

use http_types::StatusCode;

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
