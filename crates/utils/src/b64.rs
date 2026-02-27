use base64::engine::{Engine, general_purpose};

pub fn b64u_encode(content: impl AsRef<[u8]>) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(content)
}

pub fn b64u_decode(b64u: &str) -> Result<Vec<u8>> {
    general_purpose::URL_SAFE_NO_PAD
        .decode(b64u)
        .map_err(Error::InvalidBase64)
}

pub fn b64u_decode_to_string(b64u: &str) -> Result<String> {
    let bytes = b64u_decode(b64u)?;
    String::from_utf8(bytes).map_err(Error::InvalidUtf8)
}

// region:    --- Error

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidBase64(base64::DecodeError),
    InvalidUtf8(std::string::FromUtf8Error),
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(
        &self,
        fmt: &mut core::fmt::Formatter,
    ) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Error::InvalidBase64(error) => {
                write!(fmt, "invalid base64url content: {error}")
            }
            Error::InvalidUtf8(error) => {
                write!(fmt, "invalid utf-8 content: {error}")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::InvalidBase64(error) => Some(error),
            Error::InvalidUtf8(error) => Some(error),
        }
    }
}
// endregion: --- Error Boilerplate

// endregion: --- Error

#[cfg(test)]
mod tests {
    use super::{Error, b64u_decode_to_string, b64u_encode};

    #[test]
    fn decodes_valid_utf8_payload() {
        let encoded = b64u_encode("hello");
        let decoded =
            b64u_decode_to_string(&encoded).expect("valid base64url utf-8 payload");
        assert_eq!(decoded, "hello");
    }

    #[test]
    fn reports_invalid_base64_payload() {
        let result = b64u_decode_to_string("%%%");
        assert!(matches!(result, Err(Error::InvalidBase64(_))));
    }

    #[test]
    fn reports_invalid_utf8_payload() {
        let encoded = b64u_encode([0xff, 0xfe, 0xfd]);
        let result = b64u_decode_to_string(&encoded);
        assert!(matches!(result, Err(Error::InvalidUtf8(_))));
    }
}
