#![doc = "Error types."]
use http::StatusCode;
use std::fmt::{Debug, Display, Formatter};

/// [Custom error codes and descriptions](https://atproto.com/specs/xrpc#custom-error-codes-and-descriptions)
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

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum XrpcErrorKind<E>
where
    E: Debug,
{
    Custom(E),
    Undefined(ErrorResponseBody),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XrpcError<E>
where
    E: Debug,
{
    pub status: StatusCode,
    pub error: Option<XrpcErrorKind<E>>,
}

impl<E> Display for XrpcError<E>
where
    E: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.status))?;
        if let Some(error) = &self.error {
            f.write_str(" (")?;
            match error {
                XrpcErrorKind::Custom(err) => {
                    err.fmt(f)?;
                }
                XrpcErrorKind::Undefined(err) => {
                    if let Some(e) = &err.error {
                        f.write_fmt(format_args!("`{}` {:?}", e, err.message))?;
                    }
                }
            }
            f.write_str(")")?;
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error<E>
where
    E: Debug,
{
    #[error("XRPC response error: {0}")]
    XrpcResponse(XrpcError<E>),
    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] http::Error),
    #[error("HTTP client error: {0}")]
    HttpClient(Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("serde_json error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("serde_qs error: {0}")]
    SerdeQs(#[from] serde_qs::Error),
}
