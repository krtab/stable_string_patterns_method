mod private {
    pub trait Sealed {}
}
use crate::private::Sealed;

macro_rules! trait_methods_decl {
    (@ $fn_name:ident, $ret:ty) => {
        fn $fn_name(self, haystack: &str) ->$ret;
    };

    ($(
        ($fn_name:ident, $ret:ty)
    ),*) => {
        $(
            trait_methods_decl!(@ $fn_name, $ret);
        )*
    }
}

macro_rules! trait_methods_impl {
    (@ $fn_name:ident, $ret:ty) => {
        fn $fn_name(self, haystack: &str) -> $ret {
            str::$fn_name(haystack,self)
        }
    };

    ($(
        ($fn_name:ident, $ret:ty)
    ),*) => {
        $(
            trait_methods_impl!(@ $fn_name, $ret);
        )*
    }
}

macro_rules! impl_searchable {

    (@impl (
        $($generics:tt)*)
        $t:ty
        $(where $($bounds:tt)+)?
        {
            $trait_name:ty ;
            ($(
                ($fn_name:ident, $ret:ty)
            ),*) 
        }) => {

                impl $($generics)* Sealed for $t $(where $($bounds)+)? {}

                impl $($generics)* $trait_name for $t $(where $($bounds)+)? {
                   $(
                        trait_methods_impl!(
                            ($fn_name, $ret);
                        )*;
                    )*
                }
            };

    ($t:ident where $($bounds:tt)+ ; $args:tt) => {
        impl_searchable!(@impl (<$t>) $t where $($bounds)+ $args);
    };

    ($t:ty ; $args:tt) => {
        impl_searchable!(@impl () $t $args);
    };
}

macro_rules! gen_all {
    (
        $trait_name:ty
        {
            $($spec:tt)*
        }
        {
            $fn_spec:tt
        }
    ) => {
        pub trait $trait_name: Sealed {
                trait_methods_decl!(
                    $fn_spec
                );
        }
        
        $(
            impl_searchable!(
                $spec
                {
                    $trait_name ;
                    $fn_spec
                }
            )
        )*


    };
}

gen_all!(
    Searchable
    {
        char;
        &str;
        F where F: FnMut(char) -> bool;
    }
    {
        (find, Option<usize>)
    }
);
