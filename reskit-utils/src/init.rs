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
    use std::sync::Mutex;
    use linkme::distributed_slice;
    use lazy_static::lazy_static;
    use crate::{INITS, init_once};

    #[test]
    fn test_init() {
        assert_eq!(1, INITS.len());
        lazy_static!{
            pub static ref FOO: Mutex<i8> = Mutex::new(0);
        }
        #[distributed_slice(INITS)]
        fn init_test() {
            let mut num = FOO.lock().unwrap();
            *num += 1;
        }
        assert_eq!(1, INITS.len());
        init_once();
        init_once();
        assert_eq!(*FOO.lock().unwrap(), 1);
    }
}
