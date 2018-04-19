use std::fmt::{self, Display};

use failure::{Context, Fail, Backtrace};
use ::MailType;

pub const UNKNOWN: &str = "<unknown>";
pub const UTF_8: &str = "utf-8";
pub const US_ASCII: &str = "us-ascii";

#[derive(Copy, Clone, Debug, Fail, PartialEq, Eq, Hash)]
pub enum EncodingErrorKind {
    #[fail(display = "expected <{}> text encoding {} got ",
        expected_encoding, got_encoding)]
    InvalidTextEncoding {
        expected_encoding: &'static str,
        //TODO[failure >= 0.2] make it Optional remove `UNKNOWN`
        got_encoding: &'static str
    },

    #[fail(display = "hard line length limit breached (>= 998 bytes without CRLF)")]
    HardLineLengthLimitBreached,

    #[fail(display = "data can not be encoded with the {} encoding", encoding)]
    NotEncodable {
        encoding: &'static str,
    },

    #[fail(display = "malformed data")]
    Malformed,

    #[fail(display = "the mail body data cannot be accessed")]
    AccessingMailBodyFailed,

    #[fail(display = "{}", kind)]
    Other { kind: &'static str }

    //ErrorKinds potentially needed when using this wrt. to decoding the mail encoding
    //UnsupportedEncoding { encoding: &'static str }
}

// A error wrt. the used encoding. i.e. the encoding of the encoding or the
// decoding of the encoding, so it's meant for the current encoding and the
// future decoding features
#[derive(Debug)]
pub struct EncodingError {
    inner: Context<EncodingErrorKind>,
    mail_type: Option<MailType>,
    str_context: Option<String>,
    place: Option<Place>
}

#[derive(Debug)]
pub enum Place {
    Header { name: &'static str },
    Body
}

impl EncodingError {
    pub fn kind(&self) -> EncodingErrorKind {
        *self.inner.get_context()
    }

    pub fn mail_type(&self) -> Option<MailType> {
        self.mail_type
    }

    pub fn str_context(&self) -> Option<&str> {
        self.str_context.as_ref().map(|s| &**s)
    }

    pub fn set_str_context<I>(&mut self, ctx: I)
        where I: Into<String>
    {
        self.str_context = Some(ctx.into());
    }

    pub fn with_str_context<I>(mut self, ctx: I) -> Self
        where I: Into<String>
    {
        self.set_str_context(ctx);
        self
    }

    pub fn with_place_or_else<F>(mut self, func: F) -> Self
        where F: FnOnce() -> Option<Place>
    {
        if self.place.is_none() {
            self.place = func();
        }
        self
    }

    pub fn with_mail_type_or_else<F>(mut self, func: F) -> Self
        where F: FnOnce() -> Option<MailType>
    {
        if self.mail_type.is_none() {
            self.mail_type = func();
        }
        self
    }
}

impl From<EncodingErrorKind> for EncodingError {
    fn from(ctx: EncodingErrorKind) -> Self {
        EncodingError::from(Context::new(ctx))
    }
}

impl From<Context<EncodingErrorKind>> for EncodingError {
    fn from(inner: Context<EncodingErrorKind>) -> Self {
        EncodingError {
            inner,
            mail_type: None,
            str_context: None,
            place: None
        }
    }
}

impl From<(EncodingErrorKind, MailType)> for EncodingError {
    fn from((ctx, mail_type): (EncodingErrorKind, MailType)) -> Self {
        EncodingError::from((Context::new(ctx), mail_type))
    }
}

impl From<(Context<EncodingErrorKind>, MailType)> for EncodingError {
    fn from((inner, mail_type): (Context<EncodingErrorKind>, MailType)) -> Self {
        EncodingError {
            inner,
            mail_type: Some(mail_type),
            str_context: None,
            place: None
        }
    }
}

impl Fail for EncodingError {

    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for EncodingError {

    fn fmt(&self, fter: &mut fmt::Formatter) -> fmt::Result {
        if let Some(mail_type) = self.mail_type() {
            write!(fter, "[{:?}]", mail_type)?;
        } else {
            write!(fter, "[<no_mail_type>]")?;
        }
        Display::fmt(&self.inner, fter)
    }
}


#[macro_export]
macro_rules! ec_bail {
    (kind: $($tt:tt)*) => ({
        return Err($crate::error::EncodingError::from(
            $crate::error::EncodingErrorKind:: $($tt)*).into())
    });
    (mail_type: $mt:expr, kind: $($tt:tt)*) => ({
        return Err($crate::error::EncodingError::from((
            $crate::error::EncodingErrorKind:: $($tt)*,
            $mt
        )).into())
    });
}

#[cfg(test)]
mod test {

    #[test]
    fn bail_compiles_v1() {
        let func = || -> Result<(), ::error::EncodingError> {
            ec_bail!(kind: Other { kind: "test"});
            #[allow(unreachable_code)] Ok(())
        };
        assert!((func)().is_err());
    }

    #[test]
    fn bail_compiles_v2() {
        fn mail_type() -> ::MailType { ::MailType::Internationalized }
        let func = || -> Result<(), ::error::EncodingError> {
            ec_bail!(mail_type: mail_type(), kind: Other { kind: "testicle" });
            #[allow(unreachable_code)] Ok(())
        };
        assert!((func)().is_err());
    }
}