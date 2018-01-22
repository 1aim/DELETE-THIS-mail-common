pub use soft_ascii_string::{ SoftAsciiStr as _SoftAsciiStr };


/// Defines a new header types with given type name, filed name and component
/// Note that the name is not checked/validated, it has to be ascii, a valid
/// header field name AND has to comply with the naming schema (each word
/// seperated by `'-'` starts with a capital letter and no cappital letter
/// follow, e.g. "Message-Id" is ok but "Message-ID" isn't).
///
/// This macro will create a test which will check if the used field names
/// are actually valid and appears only once (_per def_header macro call_)
/// so as long as test's are run any invalid name will be found.
///
/// Note that even if a invalid name was used and test where ignored/not run
/// this will _not_ cause an rust safety issue, but can still cause bugs under
/// some circumstances (e.g. if you have multiple differing definitions of the
/// same header with different spelling (at last one failed the test) like e.g.
/// when you override default implementations of fields).
///
/// The macros expects following items:
///
/// 1. `test_name`, which is the name the auto-generated test will have
/// 2. `scope`, the scope all components are used with, this helps with some
///    name collisions. Use `self` to use the current scope.
/// 3. a list of header definitions consisting of:
///
///    1. `1` or `+`, stating wether the header can appear at most one time (1) or more times (+)
///       (not that only `Date`+`From` are required headers, no other can be made into such)
///    2. `<typename>` the name the type of the header will have, i.e. the name of a zero-sized
///       struct which will be generated
///    3. `unchecked` a hint to make people read the documentation and not forget the the
///       folowing data is `unchecked` / only vaidated in the auto-generated test
///    4. `"<header_name>"` the header name in a syntax using `'-'` to serperate words,
///       also each word has to start with a capital letter and be followed by lowercase
///       letters additionaly to being a valid header field name. E.g. "Message-Id" is
///       ok, but "Message-ID" is not. (Note that header field name are on itself ignore
///       case, but by enforcing a specific case in the encoder equality checks can be
///       done on byte level, which is especially usefull for e.g. placing them as keys
///       into a HashMap or for performance reasons.
///    5. `<component>` the name of the type to use ing `scope` a the component type of
///       the header. E.g. `Unstructured` for an unstructured header field (which still
///       support Utf8 through encoded words)
///    6. `None`/`<ident>`, None or the name of a validator function (if there is one).
///       This function is called before encoding with the header map as argument, and
///       can cause a error. Use this to enfore contextual limitations like having a
///       `From` with multiple mailboxes makes `Sender` an required field.
///
/// # Example
///
/// ```norun
/// def_headers! {
///     // the name of the auto-generated test
///     test_name: validate_header_names,
///     // the scope from which all components should be imported
///     // E.g. `DateTime` refers to `components::DateTime`.
///     scope: components,
///     // definitions of the headers
///     1 Date,     unchecked { "Date"          },  DateTime,       None,
///     1 From,     unchecked { "From"          },  MailboxList,    validator_from,
///     1 Subject,  unchecked { "Subject"       },  Unstructured,   None,
///     + Comments, unchecked { "Comments"      },  Unstructured,   None,
/// }
/// ```
#[macro_export]
macro_rules! def_headers {
    (
        test_name: $tn:ident,
        scope: $scope:ident,
        $($multi:tt $name:ident, unchecked { $hname:tt }, $component:ident, $validator:ident),+
    ) => (
        $(
            pub struct $name;

            impl $crate::headers::Header for  $name {
                const MAX_COUNT_EQ_1: bool = def_headers!(_PRIV_boolify $multi);
                type Component = $scope::$component;

                fn name() -> $crate::headers::HeaderName {
                    let as_str: &'static str = $hname;
                    $crate::headers::HeaderName::from_ascii_unchecked( as_str )
                }

                const CONTEXTUAL_VALIDATOR:
                    Option<fn(&$crate::headers::HeaderMap) -> $crate::error::Result<()>> =
                        def_headers!{ _PRIV_mk_validator $validator };
            }
        )+

        $(
            def_headers!{ _PRIV_impl_marker $multi $name }
        )+

        //TODO warn if header type name and header name diverges
        // (by stringifying the type name and then ziping the
        //  array of type names with header names removing
        //  "-" from the header names and comparing them to
        //  type names)


        #[cfg(test)]
        const HEADER_NAMES: &[ &str ] = &[ $(
            $hname
        ),+ ];

        #[test]
        fn $tn() {
            use std::collections::HashSet;
            use $crate::codec::EncodableInHeader;

            let mut name_set = HashSet::new();
            for name in HEADER_NAMES {
                if !name_set.insert(name) {
                    panic!("name appears more than one time in same def_headers macro: {:?}", name);
                }
            }
            fn can_be_trait_object<EN: EncodableInHeader>( v: Option<&EN> ) {
                let _ = v.map( |en| en as &EncodableInHeader );
            }
            $(
                can_be_trait_object::<$scope::$component>( None );
            )+
            for name in HEADER_NAMES {
                let res = $crate::headers::HeaderName::validate_name(
                    $crate::headers::_SoftAsciiStr::from_str(name).unwrap()
                );
                if res.is_err() {
                    panic!( "invalid header name: {:?} ({:?})", name, res.unwrap_err() );
                }
            }
        }
    );
    (_PRIV_mk_validator None) => ({ None });
    (_PRIV_mk_validator $validator:ident) => ({ Some($validator) });
    (_PRIV_boolify +) => ({ false });
    (_PRIV_boolify 1) => ({ true });
    (_PRIV_boolify $other:tt) => (
        compiler_error!( "only `1` (for singular) or `+` (for multiple) are valid" )
    );
    ( _PRIV_impl_marker + $name:ident ) => (
        //do nothing here
    );
    ( _PRIV_impl_marker 1 $name:ident ) => (
        impl $crate::headers::SingularHeaderMarker for $name {}
    );
}
