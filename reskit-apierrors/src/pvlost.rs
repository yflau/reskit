use std::fmt::{self, Debug, Display};
use std::convert::TryFrom;

use anyhow;

/// PV Lost code, describe the pv status and use code to identify the lost reason.
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum PVLost {
    /// 0 Successful
    /// 
    /// Response header `pvlost: 0` return when the PV is successful, that is not lost.
    Successful = 0,

    /// 1 Remote Error
    /// 
    /// Response header `pvlost: 1` return when the PV is lost because of remote error.
    RemoteError = 1,

    /// 2 Local Error
    /// 
    /// Response header `pvlost: 1` return when the PV is lost because of local error.
    LocalError = 2,
}

impl PVLost {
    /// Returns `true` if the pvlost == 0.
    ///
    /// If this returns `true` it indicates that the request was successfully
    /// received, understood, and accepted.
    pub fn is_success(&self) -> bool {
        let num: u8 = self.clone().into();
        num == 0
    }

    /// The description for a given pvlost code
    pub fn description(&self) -> &'static str {
        match self {
            PVLost::Successful => "Successful",
            PVLost::RemoteError => "Remote Error",
            PVLost::LocalError => "Local Error",
        }
    }
}

impl From<PVLost> for u8 {
    fn from(code: PVLost) -> u8 {
        code as u8
    }
}

impl TryFrom<&str> for PVLost {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "0" => Ok(PVLost::Successful),
            "1" => Ok(PVLost::RemoteError),
            "2" => Ok(PVLost::LocalError),
            _ => anyhow::bail!("Invalid pvlost code"),
        }
    }
}

impl TryFrom<u8> for PVLost {
    type Error = anyhow::Error;

    fn try_from(num: u8) -> Result<Self, Self::Error> {
        match num {
            0 => Ok(PVLost::Successful),
            1 => Ok(PVLost::RemoteError),
            2 => Ok(PVLost::LocalError),
            _ => anyhow::bail!("Invalid pvlost code"),
        }
    }
}

impl PartialEq<PVLost> for u8 {
    fn eq(&self, other: &PVLost) -> bool {
        *self == *other as u8
    }
}

impl PartialEq<u8> for PVLost {
    fn eq(&self, other: &u8) -> bool {
        *self as u8 == *other
    }
}

impl Debug for PVLost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", *self as u8, self.description())
    }
}

impl Display for PVLost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", *self as u8, self.description())
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::PVLost;
    use serde_crate::de::{Error as DeError, Unexpected, Visitor};
    use serde_crate::{Deserialize, Deserializer, Serialize, Serializer};
    use std::fmt;

    impl Serialize for PVLost {
        fn serialize<S>(&self, serializer: S) -> anyhow::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let value: u8 = *self as u8;
            serializer.serialize_u8(value)
        }
    }

    struct PVLostU8Visitor;

    impl<'de> Visitor<'de> for PVLostU8Visitor {
        type Value = PVLost;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a u8 representing the pvlost code")
        }

        fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            self.visit_u8(v as u8)
        }

        fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            self.visit_u8(v as u8)
        }

        fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            self.visit_u8(v as u8)
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            self.visit_u8(v as u8)
        }

        fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            use std::convert::TryFrom;
            match PVLost::try_from(v) {
                Ok(pvlost) => Ok(pvlost),
                Err(_) => Err(DeError::invalid_value(
                    Unexpected::Unsigned(v as u64),
                    &self,
                )),
            }
        }

        fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            self.visit_u8(v as u8)
        }

        fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            self.visit_u8(v as u8)
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            self.visit_u8(v as u8)
        }
    }

    impl<'de> Deserialize<'de> for PVLost {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(PVLostU8Visitor)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};
    use super::PVLost;
    
    #[test]
    fn test_serde_as_u8() -> Result<(), serde_json::Error> {
        let pvlost: PVLost = serde_json::from_str("2")?;
        assert_eq!(PVLost::LocalError, pvlost);
        assert_eq!(
            Some(2),
            serde_json::to_value(&PVLost::LocalError)?.as_u64()
        );
        Ok(())
    }

    #[test]
    fn test_pvlost() {
        assert_eq!(PVLost::Successful as i32, 0);
        assert_eq!(PVLost::RemoteError as i32, 1);
        assert_eq!(PVLost::LocalError as i32, 2);
        let result = PVLost::try_from("1");
        assert!(matches!(result, Ok(PVLost::RemoteError)));
        let result = PVLost::try_from("invalid");
        match result {
            Ok(..) => assert!(false, "got an OK, expected an error"),
            Err(..) => {},
        }
        let result = PVLost::try_from(2);
        assert!(matches!(result, Ok(PVLost::LocalError)));
        let result = PVLost::try_from(3);
        match result {
            Ok(..) => assert!(false, "got an OK, expected an error"),
            Err(..) => {},
        }
        let result: Result<PVLost, anyhow::Error> = 2u8.try_into();
        assert!(matches!(result, Ok(PVLost::LocalError)));
        let result: Result<PVLost, anyhow::Error> = "0".try_into();
        assert!(matches!(result, Ok(PVLost::Successful)));
    }
}
