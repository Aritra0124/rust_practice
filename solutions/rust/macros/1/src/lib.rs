#[macro_export]
macro_rules! hashmap {
    () =>{
        ::std::collections::HashMap::new()
    };
    ($($key:expr => $value:expr) ,* $(,)?) => {
        {
            let mut temp_map = ::std::collections::HashMap::new();
            $(
                temp_map.insert($key, $value);
            )+
             temp_map
        }
    };
}

/// This module contains doctests, which allows writing tests where a code
/// snippet is supposed to fail to compile. These tests also have "ignore"
/// attributes, make sure to remove them when solving this exercise locally.
pub mod compile_fail_tests;
