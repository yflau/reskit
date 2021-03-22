use linkme::distributed_slice;

#[distributed_slice]
pub static INITS: [fn()] = [..];

pub fn init_now() {
    for f in INITS {
        f()
    }
}

#[cfg(test)]
mod test {
    use linkme::distributed_slice;
    use crate::{INITS, init_now};

    #[test]
    fn test_init() {
        assert_eq!(1, INITS.len());
        static mut FOO: i8 = 0;
        #[distributed_slice(INITS)]
        fn init_test() {
            unsafe{
                FOO = 1;
            }
        }
        assert_eq!(1, INITS.len());
        init_now();
        unsafe{
            assert_eq!(FOO, 1);
        }      
    }
}
