// trace_macros!(true);

#[macro_export]
macro_rules! folder {
    (
        $mname:ident , $(
            enum $name:ident $func:ident {
                $( $vname:ident ($($vtype:ty),*) ),*
                     }
        )*
    ) => {
        mod $mname {
            $(
                pub enum $name {

                    $(
                        $vname($($vtype),*)
                    ),*
                    // $(
                    //     $vname
                    // ),*

                }
            )*

            pub trait Folder { $(
                fn $func(&mut self, me: $name) -> $name {
                    me
                }
            )*}
        }
    };
}

