#[macro_use]
extern crate shadow_rs;

shadow!(build);

mod macros;
mod init;

pub use init::{INITS, init_once};

#[cfg(test)]
mod test {
    #[test]
    fn test_version() {
        shadow!(build);
        assert_eq!(build::PKG_VERSION, "0.1.0"); 
    }
}