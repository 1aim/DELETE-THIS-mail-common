#![recursion_limit="256"]
#[macro_use]
extern crate failure;
#[macro_use]
extern crate nom;
extern crate owning_ref;
extern crate chrono;
extern crate mime;
extern crate soft_ascii_string;
extern crate base64;
extern crate quoted_printable;
extern crate idna;
extern crate quoted_string;
extern crate media_type_impl_utils;
extern crate percent_encoding;
extern crate vec1;
extern crate serde;

//NOTE: this would be worth it's own independent crate for utility macros
#[macro_use]
pub mod macros;
#[macro_use]
pub mod utils;
mod mail_type;
#[macro_use]
pub mod error;
pub mod grammar;
//NOTE: encoder is in the order _above_ bind, i.e. bind can import the encoder,
//  but the encoder should not import anything from bind!
#[cfg_attr(test, macro_use)]
pub mod encoder;
pub mod bind;

pub use self::mail_type::*;

#[cfg(all(not(feature="traceing"), test))]
compile_error! { "testing needs feature `traceing` to be enabled" }
