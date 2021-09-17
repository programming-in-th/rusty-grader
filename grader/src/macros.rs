#[macro_export]
macro_rules! instance {
    ($($arg:ident: $val:expr),*) => {{
        let mut instance: Instance = Default::default();
        $(instance.$arg = $val;)*
        instance
    }}
}
