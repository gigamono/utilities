/// Provides a nice way of expressing nested structs but defines all of them at the top-level.
#[macro_export]
macro_rules! nested_struct {
    // This matches the type part of a field of in a struct.
    // The "String" in the following example:
    // username (String)
    (@field_body ( $f_ty:ty )) => {};

    // This matches the nested struct part of a field.
    // The "(Person { name: String })" in the following example:
    // friend (Person {
    //     name (String)
    // })
    (@field_body ( $f_ty:ident {
        $( $f_name:ident $f_body:tt ),* $(,)?
    } ) ) => {
        // Resolve other possible nested struct fields.
        $(
            nested_struct!(@field_body $f_body);
        )*

        // But drop this struct definition at the top level.
        #[derive(Debug, Deserialize)]
        pub struct $f_ty {
            $(
                pub $f_name: nested_struct!(@field_ty $f_body)
            ),*
        }
    };

    // Needed for getting a regular struct field type.
    (@field_ty ( $f_ty:ty )) => {
        $f_ty
    };

    // Needed for getting a nested struct field type.
    (@field_ty ( $f_ty:ident {
        $( $f_name:ident $f_body:tt ),* $(,)?
    } ) ) => {
        $f_ty
    };

    // This is the entry point of the macro. It matches the top-level struct body.
    // The "Person { name (String) }" in the following example:
    // Person {
    //     name (String)
    // }
    ($ty:ident {
        $( $f_name:ident $f_body:tt ),* $(,)?
    }) => {
        // Resolve other possible nested struct fields.
        $(
            nested_struct!(@field_body $f_body);
        )*

        // But drop this struct definition at the top level.
        #[derive(Debug, Deserialize)]
        pub struct $ty {
            $(
                pub $f_name: nested_struct!(@field_ty $f_body)
            ),*
        }
    }
}
