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
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use linkme::distributed_slice;
    use crate::{INITS, init_once};

    #[test]
    fn test_init() {
        assert_eq!(1, INITS.len());
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        #[distributed_slice(INITS)]
        fn init_test() {
            COUNTER.fetch_add(1, Ordering::SeqCst);
        }
        assert_eq!(1, INITS.len());
        init_once();
        init_once();
        assert_eq!(COUNTER.load(Ordering::SeqCst), 1);
    }
}
