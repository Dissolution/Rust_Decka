#[macro_export]

#[allow(unused_macros)]
macro_rules! ok_or_continue {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                warn!("Error '{}' was ignored to continue", e);
                continue;
            }
        }
    };
}
#[allow(unused_imports)]
pub(crate) use ok_or_continue;


#[allow(unused_macros)]
macro_rules! some_or_continue {
    ($opt:expr) => {
        match $opt {
            Some(val) => val,
            None => {
                continue;
            }
        }
    };
}
#[allow(unused_imports)]
pub(crate) use some_or_continue;
