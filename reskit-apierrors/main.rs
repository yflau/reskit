use reskit_apierrors::{APIErrorMeta, AsAPIErrorMeta};

#[derive(Clone, Copy, Debug, PartialEq, AsAPIErrorMeta)]
pub enum MetaTest {
    #[apierrormeta(system="", code="0", message="Successful.", status_code=200, pvlost=0)]
    Successful,

    #[apierrormeta(system="", code="1", message="Unknown error.", status_code=500)]
    Failure,
}

fn main() {
    assert_eq!(MetaTest::Successful.message(), "Successful.");
    assert_eq!(MetaTest::Failure.code(), "1");
}
