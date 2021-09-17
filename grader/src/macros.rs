#[macro_export]
macro_rules! instance {
    ($($arg:ident: $val:expr),*) => {{
        let mut instance: Instance = Default::default();
        $(instance.$arg = $val;)*
        instance
    }}
}

#[macro_export]
macro_rules! combine_argument {
    ($($arg:expr),*) => {{
        let mut args = Vec::new();
        $(
            args.push(format!("{}", $arg));
        )*
        args
    }}
}
