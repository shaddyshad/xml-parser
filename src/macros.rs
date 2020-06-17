#[macro_export]
macro_rules! unwrap_or_else(
    ($opt:expr, $else_block:block) => {
        match $opt {
            None => $else_block,
            Some(c) => c,
        }
    }
);

#[macro_export]
macro_rules! unwrap_or_return(
    ($opt:expr, $retval:expr) => {
        unwrap_or_else!($opt, {return $retval })
    }
);