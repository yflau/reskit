
use anyhow::{anyhow, Context, Result};
use reskit_apierrors::{ERR_SUCCESS, ERR_UNKNOWN};

#[test]
fn test_context() {
    fn demo() -> Result<()>{
        Err(anyhow!("demo error"))
    }
    let result = demo().context(&*ERR_UNKNOWN).context(&*ERR_SUCCESS);
    match result {
        Err(err)=>{
            assert_eq!(format!("{}", err.root_cause()), "demo error");
            assert_eq!(format!("{}", err), "200::0:Successful.:0"); // FIXME: 需要类似Debug的调用链表示
            assert_eq!(format!("{:?}", err), "200::0:Successful.:0\n\nCaused by:\n    0: 500::1:Unexpected error.:2\n    1: demo error");
        },
        _ => {},
    }
}
