use std::ops::Deref;
use std::rc::Rc;
use std::borrow::ToOwned;

use soft_ascii_string::{SoftAsciiString, SoftAsciiStr};
use owning_ref::OwningRef;

use serde;

macro_rules! inner_impl {
    ($name:ident, $owned_form:ty, $borrowed_form:ty) => (

        /// a InnerItem is something potential appearing in Mail, e.g. an encoded word, an
        /// atom or a email address, but not some content which has to be represented
        /// as an encoded word, as such String is a suite representation,
        #[derive(Debug, Clone, Hash, Eq)]
        pub enum $name {
            Owned($owned_form),
            Shared(OwningRef<Rc<$owned_form>, $borrowed_form>)
        }

        impl $name {
            pub fn new<S: Into<$owned_form>>( data: S ) -> $name {
                $name::Owned( data.into() )
            }

            pub fn into_shared( self ) -> Self {
                match self {
                    $name::Owned( value ) =>
                        $name::Shared( OwningRef::new( Rc::new( value ) ).map( |rced| &**rced ) ),
                    v  => v
                }
            }

        }

        impl From<$owned_form> for $name {
            fn from( data: $owned_form ) -> Self {
                Self::new( data )
            }
        }

        impl Into<$owned_form> for $name {
            fn into(self) -> $owned_form {
                match self {
                    $name::Owned( owned ) => owned,
                    $name::Shared( shared ) => {
                        let as_ref: &$borrowed_form = &*shared;
                        as_ref.to_owned()
                    }
                }
            }
        }

        impl Deref for $name {
            type Target = $borrowed_form;

            fn deref( &self ) -> &$borrowed_form{
                match *self {
                    $name::Owned( ref string ) => &*string,
                    $name::Shared( ref owning_ref ) => &*owning_ref
                }
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: serde::Serializer
            {
                let borrowed: &$borrowed_form = &*self;
                let as_ref: &str = borrowed.as_ref();
                serializer.serialize_str( as_ref )
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &$name) -> bool {
                let me: &$borrowed_form = &*self;
                let other: &$borrowed_form = &*other;
                me == other
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

    )
}


inner_impl!{ InnerAscii, SoftAsciiString, SoftAsciiStr }
inner_impl!{ InnerUtf8, String, str }
//inner_impl!{ InnerOtherItem, OtherString, OtherStr }

impl InnerAscii {
    pub fn as_str( &self ) -> &str {
        match *self {
            InnerAscii::Owned( ref owned ) => owned.as_str(),
            InnerAscii::Shared( ref shared ) => shared.as_str()
        }
    }
}

impl InnerUtf8 {
    pub fn as_str( &self ) -> &str {
        match *self {
            InnerUtf8::Owned( ref owned ) => owned.as_str(),
            InnerUtf8::Shared( ref shared ) => &**shared
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn inner_ascii_item_eq() {
        let a = InnerAscii::Owned( SoftAsciiString::from_string( "same" ).unwrap() );
        let b = InnerAscii::Shared(
            OwningRef::new(
                Rc::new( SoftAsciiString::from_string( "same" ).unwrap() ) )
                .map(|v| &**v)
        );
        assert_eq!( a, b );
    }

    #[test]
    fn inner_ascii_item_neq() {
        let a = InnerAscii::Owned( SoftAsciiString::from_string( "same" ).unwrap() );
        let b = InnerAscii::Shared(
            OwningRef::new(
                Rc::new( SoftAsciiString::from_string( "not same" ).unwrap() ) )
                .map(|v| &**v)
        );
        assert_ne!( a, b );
    }

    #[test]
    fn inner_utf8_item_eq() {
        let a = InnerUtf8::Owned( String::from( "same" ) );
        let b = InnerUtf8::Shared(
            OwningRef::new(
                Rc::new( String::from( "same" ) ) )
                .map(|v| &**v)
        );
        assert_eq!( a, b );
    }

    #[test]
    fn inner_utf8_item_neq() {
        let a = InnerUtf8::Owned( String::from( "same" ) );
        let b = InnerUtf8::Shared(
            OwningRef::new(
                Rc::new( String::from( "not same" ) ) )
                .map(|v| &**v)
        );
        assert_ne!( a, b );
    }

    #[test]
    fn has_as_str() {
        use std::borrow::ToOwned;

        assert_eq!(
            "hy",
            InnerAscii::Owned( SoftAsciiStr::from_str_unchecked("hy").to_owned() ).as_str()
        );
        assert_eq!(
            "hy",
            InnerUtf8::Owned( "hy".into() ).as_str()
        );
    }
}