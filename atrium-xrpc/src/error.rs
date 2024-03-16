#![doc = "Error types."]
use http::StatusCode;
use std::fmt::{self, Debug, Display};

/// [A standard error response schema](https://atproto.com/specs/xrpc#error-responses)
///
/// ```typescript
/// export const errorResponseBody = z.object({
///   error: z.string().optional(),
///   message: z.string().optional(),
/// })
/// export type ErrorResponseBody = z.infer<typeof errorResponseBody>
/// ```
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ErrorResponseBody {
    pub error: Option<String>,
    pub message: Option<String>,
}

impl Display for ErrorResponseBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (&self.error, &self.message) {
            (Some(err), Some(msg)) => write!(f, "{err}: {msg}"),
            (Some(err), None) | (None, Some(err)) => write!(f, "{err}"),
            _ => Ok(()),
        }
    }
}

/// An enum of possible XRPC error kinds.
///
/// Error defined in Lexicon schema or other standard error.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum XrpcErrorKind<E> {
    Custom(E),
    Undefined(ErrorResponseBody),
}

impl<E: Display> Display for XrpcErrorKind<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Custom(e) => Display::fmt(&e, f),
            Self::Undefined(e) => Display::fmt(&e, f),
        }
    }
}

/// XRPC response error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XrpcError<E> {
    pub status: StatusCode,
    pub error: Option<XrpcErrorKind<E>>,
}

impl<E: Display> Display for XrpcError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.status.as_str())?;
        let Some(error) = &self.error else {
            return Ok(());
        };
        let error = error.to_string();
        if !error.is_empty() {
            write!(f, " {error}")?;
        }
        Ok(())
    }
}

/// An enum of possible error kinds.
#[derive(thiserror::Error, Debug)]
pub enum Error<E> {
    #[error("xrpc response error: {0}")]
    XrpcResponse(XrpcError<E>),
    #[error("http request error: {0}")]
    HttpRequest(#[from] http::Error),
    #[error("http client error: {0}")]
    HttpClient(Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("serde_json error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("serde_qs error: {0}")]
    SerdeQs(#[from] serde_qs::Error),
    #[error("unexpected response type")]
    UnexpectedResponseType,
}
