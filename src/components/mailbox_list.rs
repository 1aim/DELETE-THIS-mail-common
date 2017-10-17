

use error::*;
use soft_ascii_string::SoftAsciiChar;
use codec::{EncodableInHeader, EncodeHandle};

use external::vec1::Vec1;
use utils::{ HeaderTryFrom, HeaderTryInto};
use super::Mailbox;

#[derive(Debug)]
pub struct OptMailboxList( pub Vec<Mailbox> );
#[derive(Debug)]
pub struct MailboxList( pub Vec1<Mailbox> );

impl MailboxList {
    pub fn from_single( m: Mailbox ) -> Self {
        MailboxList( Vec1::new( m ) )
    }
}



impl EncodableInHeader for  OptMailboxList {

    fn encode(&self, handle: &mut EncodeHandle) -> Result<()> {
       encode_list( self.0.iter(), handle )
    }
}

//impl HeaderTryFrom<Mailbox> for OptMailboxList {
//    fn try_from( mbox: Mailbox ) -> Result<Self> {
//        Ok( OptMailboxList( vec![ mbox ] ) )
//    }
//}

//impl<T> HeaderTryFrom<T> for MailboxList
//    where T: HeaderTryInto<Mailbox>
//{
//    fn try_from( mbox: T ) -> Result<Self> {
//        let mbox = mbox.try_into()?;
//        Ok( MailboxList( Vec1::new( mbox ) ) )
//    }
//}

//TODO-RUST-RFC: allow conflicting wildcard implementations if priority is specified
// if done then we can implement it for IntoIterator instead of Vec and slice
impl<T> HeaderTryFrom<Vec<T>> for MailboxList
    where T: HeaderTryInto<Mailbox>
{
    fn try_from(vec: Vec<T>) -> Result<Self> {
        try_from_into_iter( vec )
    }
}

fn try_from_into_iter<IT>( mboxes: IT ) -> Result<MailboxList>
    where IT: IntoIterator, IT::Item: HeaderTryInto<Mailbox>
{
    let mut iter = mboxes.into_iter();
    let mut vec = if let Some( first) = iter.next() {
        Vec1::new( first.try_into()? )
    } else {
        bail!( "header needs at last one mailbox" );
    };
    for mbox in iter {
        vec.push( mbox.try_into()? );
    }
    Ok( MailboxList( vec ) )
}

macro_rules! impl_header_try_from_array {
    (_MBoxList 0) => ();
    (_MBoxList $len:tt) => (
        impl<T> HeaderTryFrom<[T; $len]> for MailboxList
            where T: HeaderTryInto<Mailbox>
        {
            fn try_from( vec: [T; $len] ) -> Result<Self> {
                //due to only supporting arrays halfheartedly for now
                let heapified: Box<[T]> = Box::new(vec);
                let vecified: Vec<_> = heapified.into();
                try_from_into_iter( vecified )
            }
        }
    );
    (_OptMBoxList $len:tt) => (
        impl<T> HeaderTryFrom<[T; $len]> for OptMailboxList
            where T: HeaderTryInto<Mailbox>
        {
            fn try_from( vec: [T; $len] ) -> Result<Self> {
                let heapified: Box<[T]> = Box::new(vec);
                let vecified: Vec<_> = heapified.into();
                let mut out = Vec::new();
                for ele in vecified.into_iter() {
                    out.push( ele.try_into()? );
                }
                Ok( OptMailboxList( out ) )
            }
        }
    );
    ($($len:tt)*) => ($(
        impl_header_try_from_array!{ _MBoxList $len }
        impl_header_try_from_array!{ _OptMBoxList $len }
    )*);
}

impl_header_try_from_array! {
     0  1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
}

//TODO also implement for phrase list
macro_rules! impl_header_try_from_tuple {
    (_MBoxList []) => (
        compiler_error!("mailbox list needs at last one element")
    );
    (_MBoxList [ $($vs:ident),* ]) => (
        impl< $($vs),* > HeaderTryFrom<( $($vs,)* )> for MailboxList
            where $($vs: HeaderTryInto<Mailbox>),*
        {
            #[allow(non_snake_case)]
            fn try_from( ($($vs,)*): ($($vs,)*) ) -> Result<Self> {
                // we use the type names as variable names,
                // not nice but it works
                //let ($($vs),*) = src;
                let mut out = Vec::new();
                $(
                    let $vs = $vs.try_into()?;
                    out.push($vs);
                )*
                Ok( MailboxList(
                    //UNWRAP_SAFE: len 0 is not implemented with the macro
                    $crate::external::vec1::Vec1::from_vec(out).unwrap()
                ) )
            }
        }
    );
    (_OptMBoxList [$($vs:ident),*]) => (
        impl< $($vs),* > HeaderTryFrom<( $($vs,)* )> for OptMailboxList
            where $($vs: HeaderTryInto<Mailbox>),*
        {
            #[allow(non_snake_case)]
            fn try_from( ($($vs,)*): ($($vs,)*) ) -> Result<Self> {
                // we use the type names as variable names,
                // not nice but it works
                //let ($($vs),*) = src;
                let mut out = Vec::new();
                $(
                    let $vs = $vs.try_into()?;
                    out.push($vs);
                )*
                Ok( OptMailboxList( out ) )
            }
        }
    );
    ($([$($vs:ident),*]),*) => ($(
        impl_header_try_from_tuple!{ _MBoxList [$($vs),*] }
        impl_header_try_from_tuple!{ _OptMBoxList [$($vs),*] }
    )*);
}

impl_header_try_from_tuple! {
    [A0],
    [A0, A1],
    [A0, A1, A2],
    [A0, A1, A2, A3],
    [A0, A1, A2, A3, A4],
    [A0, A1, A2, A3, A4, A5],
    [A0, A1, A2, A3, A4, A5, A6],
    [A0, A1, A2, A3, A4, A5, A6, A7],
    [A0, A1, A2, A3, A4, A5, A6, A7, A8],
    [A0, A1, A2, A3, A4, A5, A6, A7, A8, A9],
    [A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10],
    [A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11],
    [A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12],
    [A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13],
    [A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14],
    [A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15],
    [A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16],
    [A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17],
    [A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18],
    [A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19],
    [A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20],
    [A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20,
        A21],
    [A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20,
        A21, A22],
    [A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20,
        A21, A22, A23],
    [A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20,
        A21, A22, A23, A24],
    [A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20,
        A21, A22, A23, A24, A25],
    [A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20,
        A21, A22, A23, A24, A25, A26],
    [A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20,
        A21, A22, A23, A24, A25, A26, A27],
    [A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20,
        A21, A22, A23, A24, A25, A26, A27, A28],
    [A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20,
        A21, A22, A23, A24, A25, A26, A27, A28, A29],
    [A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20,
        A21, A22, A23, A24, A25, A26, A27, A28, A29, A30],
    [A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20,
        A21, A22, A23, A24, A25, A26, A27, A28, A29, A30, A31]
}

impl<T> HeaderTryFrom<Vec<T>> for OptMailboxList
    where T: HeaderTryInto<Mailbox>
{
    fn try_from(vec: Vec<T>) -> Result<Self> {
        let mut out = Vec::new();
        for ele in vec.into_iter() {
            out.push( ele.try_into()? );
        }
        Ok( OptMailboxList( out ) )
    }
}

impl EncodableInHeader for  MailboxList {

    fn encode(&self, handle: &mut EncodeHandle) -> Result<()> {
        encode_list( self.0.iter(), handle )
    }
}

fn encode_list<'a, I>(list_iter: I, handle: &mut EncodeHandle) -> Result<()>
    where I: Iterator<Item=&'a Mailbox>
{
    sep_for!{ mailbox in list_iter;
        sep {
            handle.write_char( SoftAsciiChar::from_char_unchecked(',') )?;
            handle.write_fws();
        };
        mailbox.encode( handle )?;
    }
    Ok( () )
}

deref0!{ +mut OptMailboxList => Vec<Mailbox> }
deref0!{ +mut MailboxList => Vec<Mailbox> }

#[cfg(test)]
mod test {
    use data::FromInput;
    use components::{ Mailbox, Email, Phrase };
    use super::*;


    ec_test! { empty_list, {
        OptMailboxList( Vec::new() )
    } => ascii => [

    ]}

    ec_test! { single, {
        MailboxList( vec1![
            Mailbox {
                display_name: Some( Phrase::from_input( "hy ho" )? ),
                email: Email::from_input( "ran@dom" )?
            },
        ] )
    } => ascii => [
        Text "hy",
        MarkFWS,
        Text " ho",
        MarkFWS,
        Text " <",
        MarkFWS,
        Text "ran",
        MarkFWS,
        Text "@",
        MarkFWS,
        Text "dom",
        MarkFWS,
        Text ">"
    ]}

    ec_test! { multiple, {
         MailboxList( vec1![
            Mailbox {
                display_name: Some( Phrase::from_input( "hy ho" )? ),
                email: Email::from_input( "nar@mod" )?
            },
            Mailbox {
                display_name: None,
                email: Email::from_input( "ran@dom" )?
            }
        ] )
    } => ascii => [
        Text "hy",
        MarkFWS,
        Text " ho",
        MarkFWS,
        Text " <",
        MarkFWS,
        Text "nar",
        MarkFWS,
        Text "@",
        MarkFWS,
        Text "mod",
        MarkFWS,
        Text ">,",
        MarkFWS,
        Text " <",
        MarkFWS,
        Text "ran",
        MarkFWS,
        Text "@",
        MarkFWS,
        Text "dom",
        MarkFWS,
        Text ">"
    ]}
}