/// Return the caller infomation as a string slice.
/// From: https://stackoverflow.com/questions/38088067/equivalent-of-func-or-function-in-rust
#[macro_export]
macro_rules! caller {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }};
    ($skip:expr) => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        let name = &name[..name.len() - 3];
        let v: Vec<_> = name.rmatch_indices("::").collect();
        let end = if $skip < 1 || v.len() <= $skip-1 {
            name.len()
        } else {
            v.get($skip-1).unwrap().0
        };
        &name[..end]
    }}
}

/// Return the caller function name as a string slice.
/// Warning: No guarante because of no  determined format of `std::any::type_name`.
/// From: https://stackoverflow.com/questions/38088067/equivalent-of-func-or-function-in-rust
#[macro_export]
macro_rules! fn_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn macros_test() {
        assert_eq!(caller!(), "reskit_utils::macros::tests::macros_test");
        assert_eq!(caller!(4), "reskit_utils::macros::tests::macros_test");
        assert_eq!(fn_name!(), "macros_test");
        fn inner() {
            assert_eq!(caller!(), "reskit_utils::macros::tests::macros_test::inner");
            assert_eq!(caller!(1), "reskit_utils::macros::tests::macros_test");
        }
        inner();
        assert_eq!(caller!(5), "reskit_utils::macros::tests::macros_test");
    }
}
