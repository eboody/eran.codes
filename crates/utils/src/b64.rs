use base64::engine::{Engine, general_purpose};
use snafu::Snafu;

pub fn b64u_encode(content: impl AsRef<[u8]>) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(content)
}

pub fn b64u_decode(b64u: &str) -> Result<Vec<u8>> {
    general_purpose::URL_SAFE_NO_PAD
        .decode(b64u)
        .map_err(|source| Error::InvalidBase64 { source })
}

pub fn b64u_decode_to_string(b64u: &str) -> Result<String> {
    let bytes = b64u_decode(b64u)?;
    String::from_utf8(bytes).map_err(|source| Error::InvalidUtf8 { source })
}

// region:    --- Error

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("invalid base64url content: {source}"))]
    InvalidBase64 { source: base64::DecodeError },
    #[snafu(display("invalid utf-8 content: {source}"))]
    InvalidUtf8 { source: std::string::FromUtf8Error },
}

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
        assert!(matches!(result, Err(Error::InvalidBase64 { .. })));
    }

    #[test]
    fn reports_invalid_utf8_payload() {
        let encoded = b64u_encode([0xff, 0xfe, 0xfd]);
        let result = b64u_decode_to_string(&encoded);
        assert!(matches!(result, Err(Error::InvalidUtf8 { .. })));
    }
}
