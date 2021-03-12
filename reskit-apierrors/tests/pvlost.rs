use std::convert::{TryFrom, TryInto};

use reskit_apierrors::{PVLost};

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
