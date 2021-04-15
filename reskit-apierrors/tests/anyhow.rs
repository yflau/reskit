
use anyhow::{anyhow, Context, Result};
use reskit_apierrors::Builtin;

#[test]
fn test_context() {
    fn demo() -> Result<()>{
        Err(anyhow!("demo error"))
    }
    let result = demo().context(Builtin::Unknown).context(Builtin::Successful);
    match result {
        Err(err)=>{
            assert_eq!(format!("{}", err.root_cause()), "demo error");
            assert_eq!(format!("{}", err), "200::0:Successful."); // FIXME: 需要类似Debug的调用链表示
            assert_eq!(format!("{:?}", err), "200::0:Successful.\n\nCaused by:\n    0: 500::1:Unexpected error.\n    1: demo error");
        },
        _ => {},
    }
}
