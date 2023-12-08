/// A trait for casting a type to another type using the `as` operator.
pub trait CastInto<To> {
    /// Returns `self as To`.
    fn cast_into(self) -> To;
}

/// A trait for casting a type from another type using the `as` operator.
pub trait CastFrom<From> {
    /// Returns `from as Self`.
    fn from_cast(from: From) -> Self;
}

impl<A, B> CastInto<A> for B
where
    A: CastFrom<B>,
{
    #[inline]
    fn cast_into(self) -> A {
        A::from_cast(self)
    }
}

impl<A> CastFrom<A> for A {
    #[inline]
    fn from_cast(from: A) -> Self {
        from
    }
}

/// Allows casting from this type to other types using
/// [`CastFrom`]/[`CastInto`].
pub trait Cast: Sized {
    /// Casts `self` to the `To` type. This may be a lossy operation.
    fn cast<To: CastFrom<Self>>(self) -> To;
}

impl<A> Cast for A {
    #[inline]
    fn cast<To: CastFrom<Self>>(self) -> To {
        To::from_cast(self)
    }
}

macro_rules! impl_cast_from {
    ($from:ident, [$($to:ident),+]) => {
        $(impl_cast_from!($from, $to);)+
    };
    ($from:ident, $to:ident) => {
        impl CastFrom<$from> for $to {
            #[doc = "```rust\n"]
            #[doc = "# use intentional::Cast;\n"]
            #[doc = concat!("let casted: ", stringify!($to), " = 1_", stringify!($from), ".cast();\n")]
            #[doc = "```\n"]
            #[inline]
            fn from_cast(from: $from) -> Self {
                from as $to
            }
        }
    };
}

impl_cast_from!(u8, i8);
impl_cast_from!(i8, [u8, usize]);
impl_cast_from!(u16, [u8, i8, i16]);
impl_cast_from!(i16, [u8, i8, u16, usize]);
impl_cast_from!(u32, [f32, usize, isize, u8, i8, u16, i16, i32]);
impl_cast_from!(i32, [f32, usize, isize, u8, i8, u16, i16, u32]);
impl_cast_from!(
    u64,
    [f32, f64, usize, isize, u8, i8, u16, i16, u32, i32, i64]
);
impl_cast_from!(
    i64,
    [f32, f64, usize, isize, u8, i8, u16, i16, u32, i32, u64]
);
impl_cast_from!(
    u128,
    [f32, f64, usize, isize, u8, i8, u16, i16, u32, i32, u64, i64, i128]
);
impl_cast_from!(
    i128,
    [f32, f64, usize, isize, u8, i8, u16, i16, u32, i32, u64, i64, u128]
);
impl_cast_from!(
    isize,
    [f32, f64, usize, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128]
);
impl_cast_from!(
    usize,
    [f32, f64, isize, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128]
);
impl_cast_from!(
    f32,
    [usize, isize, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128]
);
impl_cast_from!(
    f64,
    [usize, isize, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f32]
);
