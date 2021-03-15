use std::fmt::{Display, Result, Formatter};

use crate::{PVLost};
use http_types::{StatusCode};

pub trait APIErrorMeta {
    fn system(&self) -> &str;
    fn code(&self) -> &str;
    fn message(&self) -> &str;
    fn status_code(&self) -> StatusCode;
    fn pvlost(&self) -> PVLost;
}

#[derive(Debug)]
pub struct APIErrorClass {
    system: String,
    code: String,
    message: String, // FIXME: what about &'static str? And make APIErrorClass as immutable？
    status: StatusCode,
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