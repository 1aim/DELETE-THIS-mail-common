use error::EncodingError;

/// Trait Repesenting the buffer of a mime body payload
///
/// (e.g. a transfer encoded image or text)
///
/// Note that the `BodyBuffer` trait is mainly used to break a
/// cyclic dependency between `codec` and `mail::resource`.
/// So while all code in lower layers is generic over _one_
/// kind of BodyBuffer for all Buffers the higher layers
/// in `mail` and `mail_composition` are fixed on `Resource`.
///
pub trait BodyBuffer {

    /// Called to access the bytes in the buffer.
    ///
    /// By limiting the access to a closure passed in
    /// it enables a number of properties for implementors:
    /// - the byte slice has only to be valid for the duration of the closure,
    ///   allowing implementations for data behind a Lock which has to keep
    ///   a Guard alive during the access of the data
    /// - the implementor can directly return a error if for some
    ///   reason no data is available or the data was "somehow" corrupted
    ///
    /// # Error
    ///
    /// There are two error cases:
    /// 1. the passed in closure causes an error (e.g. an utf8 encoding error)
    /// 2. accessing the body itself causes an error (e.g. a lock got poisoned)
    ///
    /// In the later case the error should be converted to an `EncodingError`
    /// using the `AccessingMailBodyFailed` error kind.
    fn with_slice<FN, R>(&self, func: FN) -> Result<R, EncodingError>
        where FN: FnOnce(&[u8]) -> Result<R, EncodingError>;
}

/// A BodyBuf implementation based on a Vec<u8>
///
/// this is mainly used for having a simple
/// BodyBuf implementation for testing.
pub struct VecBodyBuf(pub Vec<u8>);

impl BodyBuffer for VecBodyBuf {
    fn with_slice<FN, R>(&self, func: FN) -> Result<R, EncodingError>
        where FN: FnOnce(&[u8]) -> Result<R, EncodingError>
    {
        func(self.0.as_slice())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Section<R: BodyBuffer> {
    String(String),
    BodyPayload(R)
}

impl<R> Section<R>
    where R: BodyBuffer
{
    pub fn unwrap_header(self) -> String {
        if let Section::String(res) = self {
            res
        } else {
            panic!("expected `Section::Header` got `Section::Body`")
        }
    }
    pub fn unwrap_body(self) -> R {
        if let Section::BodyPayload(res) = self {
            res
        } else {
            panic!("expected `Section::MIMEBody` got `Section::Header`")
        }
    }
}