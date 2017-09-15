macro_rules! iter_impl {
    (_REF MUT $t:ty) => (
        &mut $t
    );
    (_REF REF $t:ty) => (
        & $t
    );
    (_REF2 MUT $lt:tt $t:ty ) => (
        & $lt mut $t
    );
    (_REF2 REF $lt:tt $t:ty) => (
        & $lt $t
    );
    (_REF EXPR MUT $e:expr) => (
        &mut $e
    );
    (_REF EXPR REF $e:expr) => (
        & $e
    );
    (
        $fn_name:ident, $tp_name:ident, $mutability:tt
    ) => (
        use std::slice;
        use $crate::codec::{ MailEncoder, MailEncodable };
        use $crate::headers::{ HeaderName, HeaderMap };

        impl<E: MailEncoder> HeaderMap<E> {
            pub fn $fn_name(self: iter_impl!{ _REF $mutability Self } ) -> $tp_name<E> {
                $tp_name {
                    vec_ptr_iter: self.header_vec.$fn_name()
                }
            }
        }

        pub struct $tp_name<'a, E: MailEncoder> {
            vec_ptr_iter: slice::$tp_name<'a, (HeaderName, Box<MailEncodable<E>>)>
        }

        impl<'a, E> Iterator for $tp_name<'a, E>
            where E: MailEncoder
        {
            type Item = (HeaderName, iter_impl!{ _REF2 $mutability 'a MailEncodable<E> } );

            fn next(&mut self) -> Option<Self::Item> {
                self.vec_ptr_iter.next()
                    .map( |name_and_box| {
                        let name = name_and_box.0;
                        let reference = iter_impl!{_REF EXPR $mutability *name_and_box.1 };
                        (name, reference)
                    })
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                self.vec_ptr_iter.size_hint()
            }
        }

    );
}

mod mut_iter {
    iter_impl!{
        iter_mut, IterMut, MUT
    }
}

mod ref_iter {
    iter_impl!{
        iter, Iter, REF
    }
}

#[cfg(test)]
mod test {
    use codec::MailEncoderImpl;
    use headers::{ Subject, Comments };
    use components::Unstructured;
    use super::super::HeaderMap;

    #[test]
    fn iter_in_order() {
        let mut map: HeaderMap<MailEncoderImpl> = HeaderMap::new();
        map.insert(Comments, "A").unwrap();
        map.insert(Subject, "B").unwrap();
        map.insert(Comments, "nix C").unwrap();


        let res = map.iter()
            .map(|(name, val)| {
                let name = name.as_str();
                let text = val.downcast_ref::<Unstructured>().unwrap().as_str();
                (name, text)
            })
            .collect::<Vec<_>>();

        assert_eq!(
            &[ ("Comments", "A") , ("Subject", "B"), ("Comments", "nix C") ],
            res.as_slice()
        );
    }

    #[test]
    fn iter_mut_in_order() {
        let mut map: HeaderMap<MailEncoderImpl> = HeaderMap::new();
        map.insert(Comments, "A").unwrap();
        map.insert(Subject, "B").unwrap();
        map.insert(Comments, "nix C").unwrap();

        let res = map.iter_mut()
            .map(|(name, val)| {
                let name = name.as_str();
                let text = val.downcast_ref::<Unstructured>().unwrap().as_str();
                (name, text)
            })
            .collect::<Vec<_>>();

        assert_eq!(
            &[ ("Comments", "A") , ("Subject", "B"), ("Comments", "nix C") ],
            res.as_slice()
        );
    }
}