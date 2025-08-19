macro_rules! import {
    ($($module:ident),* $(,)?) => {
        $(
            pub mod $module;
            #[allow(unused_imports)]
            pub use $module::*;
        )*
    };
}

pub mod models;

import!(sub_client, combinator);
