#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum MailType {
    Ascii,
    Mime8BitEnabled,
    Internationalized
}

impl MailType {
    #[inline]
    pub fn is_internationalized(&self) -> bool {
        *self == MailType::Internationalized
    }
    pub fn supports_8bit_bodies( &self ) -> bool {
        use self::MailType::*;
        match *self {
            Ascii => false,
            Mime8BitEnabled => true,
            Internationalized => true
        }
    }
}
