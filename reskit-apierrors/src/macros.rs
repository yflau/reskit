#[macro_export]
macro_rules! adapt_errorspace {
    ($name:expr, $error:expr, $meta:expr $(,$mapping:expr)* ) => {
        $crate::globals::adapt_errorspace(
            $name, 
            $error, 
            $meta, 
            &[$($mapping,)*], 

            #[cfg(feature = "verbose")]
            Some(reskit_utils::caller!()),

            #[cfg(not(feature = "verbose"))]
            None,
        )
    };
}

/// adapt! 
#[macro_export]
macro_rules! adapt {
    ($error:expr, $meta:expr $(,$mapping:expr)* ) => {
        $crate::globals::adapt_errorspace(
            $crate::GLOBAL_ERRORSPACE_NAME, 
            $error, 
            $meta, 
            &[$($mapping,)*], 

            #[cfg(feature = "verbose")]
            Some(reskit_utils::caller!()),

            #[cfg(not(feature = "verbose"))]
            None,
        )
    };
}

#[macro_export]
macro_rules! force_errorspace {
    ($name:expr, $error:expr, $meta:expr $(,$mapping:expr)* ) => {
        $crate::globals::force_errorspace(
            $name, 
            $error, 
            $meta, 
            &[$($mapping,)*], 

            #[cfg(feature = "verbose")]
            Some(reskit_utils::caller!()),

            #[cfg(not(feature = "verbose"))]
            None,
        )
    };
}

#[macro_export]
macro_rules! force {
    ($error:expr, $meta:expr $(,$mapping:expr)* ) => {
        $crate::globals::force_errorspace(
            $crate::GLOBAL_ERRORSPACE_NAME, 
            $error, 
            $meta, 
            &[$($mapping,)*], 

            #[cfg(feature = "verbose")]
            Some(reskit_utils::caller!()),

            #[cfg(not(feature = "verbose"))]
            None,
        )
    };
}

#[cfg(test)]
mod tests {
    use reskit_utils::init_once;
    use crate::{Builtin, adapt_errorspace, adapt, };

    #[test]
    fn test_adapt_errorspace() {
        init_once();
        let e = adapt_errorspace!("", anyhow::anyhow!("xxx"), &Builtin::Unknown);
        assert_eq!(format!("{}", e), "500::1:Unknown error.:reskit_apierrors::macros::tests::test_adapt_errorspace->xxx");
        let e2 = adapt_errorspace!("", anyhow::anyhow!("xxx"), &Builtin::Unknown, "authcar", "rest");
        assert_eq!(format!("{}", e2), "500::1:Unknown error.:reskit_apierrors::macros::tests::test_adapt_errorspace->xxx");
    }

    #[test]
    fn test_adapt() {
        init_once();
        let e = adapt!(anyhow::anyhow!("xxx"), &Builtin::Unknown);
        assert_eq!(format!("{}", e), "500::1:Unknown error.:reskit_apierrors::macros::tests::test_adapt->xxx");
        let e2 = adapt!( anyhow::anyhow!("xxx"), &Builtin::Unknown, "authcar", "rest");
        assert_eq!(format!("{}", e2), "500::1:Unknown error.:reskit_apierrors::macros::tests::test_adapt->xxx");
    }

    #[test]
    fn test_force_errorspace() {
        init_once();
        let e = force_errorspace!("", anyhow::anyhow!("xxx"), &Builtin::Unknown);
        assert_eq!(format!("{}", e), "500::1:Unknown error.:reskit_apierrors::macros::tests::test_force_errorspace->xxx");
        let e2 = force_errorspace!("", anyhow::anyhow!("xxx"), &Builtin::Unknown, "authcar", "rest");
        assert_eq!(format!("{}", e2), "500::1:Unknown error.:reskit_apierrors::macros::tests::test_force_errorspace->xxx");
    }

    #[test]
    fn test_force() {
        init_once();
        let e = force!(anyhow::anyhow!("xxx"), &Builtin::Unknown);
        assert_eq!(format!("{}", e), "500::1:Unknown error.:reskit_apierrors::macros::tests::test_force->xxx");
        let e2 = force!( anyhow::anyhow!("xxx"), &Builtin::Unknown, "authcar", "rest");
        assert_eq!(format!("{}", e2), "500::1:Unknown error.:reskit_apierrors::macros::tests::test_force->xxx");
    }
}
