use http_types::{StatusCode};

use reskit_apierrors::{PVLost, APIErrorClass, APIErrorMeta};

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
