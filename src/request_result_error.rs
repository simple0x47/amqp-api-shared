use std::fmt;

use serde::{Deserialize, Serialize};

/// Error kinds copied from HTTP status codes
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub enum RequestResultErrorKind {
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    URITooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ImATeapot,
    MisdirectedRequest,
    UnprocessableContent,
    Locked,
    FailedDependency,
    TooEarly,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended,
    NetworkAuthenticationRequired,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestResultError {
    kind: RequestResultErrorKind,
    message: String,
}

impl RequestResultError {
    pub fn new(kind: RequestResultErrorKind, message: impl Into<String>) -> RequestResultError {
        RequestResultError {
            kind,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> RequestResultErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for RequestResultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
