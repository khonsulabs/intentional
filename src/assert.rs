use core::fmt::Debug;

/// Asserts that a type is an expected value.
pub trait Assert: Sized {
    /// The type exected to be contained.
    type Expected;

    /// Aserts that `self` is the [expected type](Self::Expected). If not a
    /// panic containing `assertion failed` will occur.
    ///
    /// This function should only be used in functions when the programmer is
    /// asserting a condition that the caller of the function has no control
    /// over.
    #[inline]
    fn assert_expected(self) -> Self::Expected {
        self.assert("assertion failed")
    }

    /// Asserts that `self` is the [expected type](Self::Expected). If not, a
    /// panic containing `msg` will occur.
    ///
    /// This function should only be used in functions when the programmer is
    /// asserting a condition that the caller of the function has no control
    /// over.
    fn assert(self, msg: &str) -> Self::Expected;
}

impl Assert for bool {
    type Expected = bool;

    #[inline]
    fn assert(self, msg: &str) -> Self::Expected {
        self || panic!("{msg}")
    }
}

impl<'a, T> Assert for &'a Option<T> {
    type Expected = &'a T;

    #[inline]
    fn assert(self, msg: &str) -> Self::Expected {
        self.as_ref().expect(msg)
    }
}

impl<'a, T> Assert for &'a mut Option<T> {
    type Expected = &'a mut T;

    #[inline]
    fn assert(self, msg: &str) -> Self::Expected {
        self.as_mut().expect(msg)
    }
}

impl<T> Assert for Option<T> {
    type Expected = T;

    #[inline]
    fn assert(self, msg: &str) -> Self::Expected {
        self.expect(msg)
    }
}

impl<T, E> Assert for Result<T, E>
where
    E: Debug,
{
    type Expected = T;

    #[inline]
    fn assert(self, msg: &str) -> Self::Expected {
        self.expect(msg)
    }
}

impl<'a, T, E> Assert for &'a Result<T, E>
where
    E: Debug,
{
    type Expected = &'a T;

    #[inline]
    fn assert(self, msg: &str) -> Self::Expected {
        self.as_ref().expect(msg)
    }
}

impl<'a, T, E> Assert for &'a mut Result<T, E>
where
    E: Debug,
{
    type Expected = &'a T;

    #[inline]
    fn assert(self, msg: &str) -> Self::Expected {
        self.as_mut().expect(msg)
    }
}

#[macro_export]
macro_rules! intentional {
    ($expr:expr) => {
        $crate::Assert::assert_expected($expr)
    };
    ($expr:expr, $msg:expr) => {
        $crate::Assert::assert($expr, $msg)
    };
}

#[test]
fn intentional() {
    struct NonCopy;
    let option = Some("string");
    let _: &str = intentional!(option);
    let _: &str = intentional!(option, "msg");
    let result = Ok::<_, ()>(NonCopy);
    let _: &NonCopy = intentional!(&result);
    let _ = intentional!(result);
    intentional!(true);
}
