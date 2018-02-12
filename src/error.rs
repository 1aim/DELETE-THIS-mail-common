use std::io;

use base64;
use quoted_printable;
use idna::uts46::{ Errors as PunyCodeErrors };
use mime::AnyMediaType;
use ::MailType;

// we do not wan't dependencies to have to import error_chain
// just to have some of the additional error chaining functions
pub use error_chain::ChainedError;

#[allow(unused_doc_comment)]
error_chain! {

    foreign_links {
        Io( io::Error );
        DecodeBase64(base64::DecodeError);
        DecodeQuotedPrintable(quoted_printable::QuotedPrintableError);
    }

    errors {

        HeaderTypeMixup {
            description(concat!(
                "multiple header types with the same name, which differ in quantity or validator"
            ))
        }

        InvalidInput(for_usage_in: &'static str, input: String, mail_type: MailType) {
            description("the given input was invalid for the given use case")
            display("the input is invalid for usage wrt. {}. Input: {:?} (mt: {:?})",
                for_usage_in, input, mail_type)
        }

        /// the contextual validation of the header using the headers validator failed
        ///
        /// This can e.g. happen if the header contains a multi-mailbox resent-from, but
        /// no resent-sender header.
        HeaderValidationFailure {
            description("validation of header in HeaderMap failed")
        }

        HeaderComponentEncodingFailure {
            description("encoding header component failed")
        }

        /// adding a header to the header map failed
        ///
        /// use `.cause()` to gain more information about why/how it did
        /// fail.
        FailedToAddHeader(name: &'static str) {
            description("failed to add a header filed to the header map")
            display("failed to add the field {:?} to the header map", name)
        }

        HardLineLengthLimitBreached {
            description("the line length is limited to 998 bytes (excluding tailing \r\n)")
        }


        MalformedEncodedWord(word: String) {
            description("the encoded word is not well-formed")
            display("the encoded word {:?} is not well-formed", word)

        }

        PunyCodeingDomainFailed( errors: PunyCodeErrors ) {
            description( "using puny code to encode the domain failed" )
        }

        InvalidLineBrake {
            description( "the chars '\\r', '\\n' can only appear as \"\\r\\n\"")
        }

        NonUtf8Body {
            description("can not convert body to string as it contains non utf8 chars")
        }

        Utf8InHeaderRequiresInternationalizedMail {
            description("to use utf-8 in a header a internationalized mail is needed")
        }

        InvalidHeaderName(name: String) {
            description( "given header name is not valid" )
            display( "{:?} is not a valid header name", name )
        }

        //------------------------------------- DEPRECATED --------------------------------//


        //TODO mv to `mail-codec` && `mail-codec-composition`
        NeedAtLastOneBodyInMultipartMail {

        }

        //TODO mv to `mail-codec`
        GeneratingMimeFailed {

        }


        //TODO mv to `mail-codec`
        ContentTypeAndBodyIncompatible {
            description( concat!(
                "given content type is incompatible with body,",
                "e.g. using a non multipart mime with a multipart body" ) )
        }


        //TODO mv to `mail-codec`
        Invalide7BitValue( byte: u8 ) {
            description( "the byte is not valid in 7bit (content transfer) encoding" )
        }

        //TODO mv to `mail-codec`
        Invalide8BitValue( val: u8 ) {
            description( "the byte is not valid in 8bit (content transfer) encoding" )
        }

        //TODO mv to `mail-codec`
        Invalide7BitSeq( byte: u8 ) {
            description( "the byte seq is not valid in 7bit (content transfer) encoding" )
        }

        //TODO mv to `mail-codec`
        Invalide8BitSeq( val: u8 ) {
            description( "the byte seq is not valid in 8bit (content transfer) encoding" )
        }

        //TODO mv to `mail-codec`
        NotMultipartMime( mime: AnyMediaType ) {
            description( "expected a multipart mime for a multi part body" )
            display( _self ) -> ( "{}, got: {}", _self.description(), mime )
        }

    }
}
