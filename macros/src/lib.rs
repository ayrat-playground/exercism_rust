use std::collections::HashMap;

#[macro_export]
macro_rules! hashmap {
    (
        $(
            $x:expr => $y:expr$(,)*
        )*
    ) => {
        {

            let mut hash = HashMap::new();

            $(
                hash.insert($x, $y);
            )*

            hash
        }
    };
}
