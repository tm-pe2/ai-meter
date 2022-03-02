macro_rules! quick_impl {
    (From<$from:ty> for $for:ty) => {
        impl From<$from> for $for {
            fn from(err: $from) -> Self {
                Self::new(err)
            }
        }
    };

    (From<$from:ty> for $for:ty, $variant:path) => {
        impl From<$from> for $for {
            fn from(_: $from) -> Self {
                $variant
            }
        }
    };
}

pub(crate) use quick_impl;
