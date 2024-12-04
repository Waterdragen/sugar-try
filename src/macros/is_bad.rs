#[macro_export]
macro_rules! is_bad {
    ($enum_:expr) => {{
        use $crate::__private::__BadIndex;
        $enum_.__is_bad()
    }};
}