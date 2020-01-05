
#[macro_export]
macro_rules! set (
    { $($x:expr),+ } => {
        {
            let mut m = ::std::collections::HashSet::new();
            $(
                m.insert($x);
            )+
                m
        }
    };
);
