use std::sync::Once;

use linkme::distributed_slice;

static INIT_ONCE: Once = Once::new();

#[distributed_slice]
pub static INITS: [fn()] = [..];

pub fn init_once() {
    INIT_ONCE.call_once(|| {
        for f in INITS {
            f()
        }
    });
}

#[cfg(test)]
mod test {
    use linkme::distributed_slice;
    use crate::{INITS, init_once};

    #[test]
    fn test_init() {
        assert_eq!(1, INITS.len());
        static mut FOO: i8 = 0;
        #[distributed_slice(INITS)]
        fn init_test() {
            unsafe{
                FOO += 1;
            }
        }
        assert_eq!(1, INITS.len());
        init_once();
        init_once();
        unsafe{
            assert_eq!(FOO, 1);
        }      
    }
}
