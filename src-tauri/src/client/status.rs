use serde::{Deserialize, Serializer};

#[derive(Debug, Deserialize, ts_rs::TS)]
#[ts(export)]
#[ts(export_to = "response_status.ts")]
pub enum ResponseStatus {
    // Information
    Continue,
    SwitchingProtocols,
    Processing,
    EarlyHints,

    // Successful Operation
    Ok,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    IMUsed,

    // Redirect
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    TemporaryRedirect,
    PermanentRedirect,

    // Client Error
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
    ExpectationFailed,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    TooEarly,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,

    // Server Error
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HTTPVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    BandwidthLimitExceeded,
    NotExtended,
    NetworkAuthenticationRequired,
}

impl ResponseStatus {
    pub fn from(code: u16) -> ResponseStatus {
        match code {
            100 => Self::Continue,
            101 => Self::SwitchingProtocols,
            102 => Self::Processing,
            103 => Self::EarlyHints,

            200 => Self::Ok,
            201 => Self::Created,
            202 => Self::Accepted,
            203 => Self::NonAuthoritativeInformation,
            204 => Self::NoContent,
            205 => Self::ResetContent,
            206 => Self::PartialContent,
            207 => Self::MultiStatus,
            208 => Self::AlreadyReported,
            226 => Self::IMUsed,

            300 => Self::MultipleChoices,
            301 => Self::MovedPermanently,
            302 => Self::Found,
            303 => Self::SeeOther,
            304 => Self::NotModified,
            305 => Self::UseProxy,
            307 => Self::TemporaryRedirect,
            308 => Self::PermanentRedirect,

            400 => Self::BadRequest,
            401 => Self::Unauthorized,
            402 => Self::PaymentRequired,
            403 => Self::Forbidden,
            404 => Self::NotFound,
            405 => Self::MethodNotAllowed,
            406 => Self::NotAcceptable,
            407 => Self::ProxyAuthenticationRequired,
            408 => Self::RequestTimeout,
            409 => Self::Conflict,
            410 => Self::Gone,
            411 => Self::LengthRequired,
            412 => Self::PreconditionFailed,
            413 => Self::PayloadTooLarge,
            414 => Self::URITooLong,
            415 => Self::UnsupportedMediaType,
            416 => Self::RangeNotSatisfiable,
            417 => Self::ExpectationFailed,
            421 => Self::MisdirectedRequest,
            422 => Self::UnprocessableEntity,
            423 => Self::Locked,
            424 => Self::FailedDependency,
            425 => Self::TooEarly,
            426 => Self::UpgradeRequired,
            428 => Self::PreconditionRequired,
            429 => Self::TooManyRequests,
            431 => Self::RequestHeaderFieldsTooLarge,
            451 => Self::UnavailableForLegalReasons,

            500 => Self::InternalServerError,
            501 => Self::NotImplemented,
            502 => Self::BadGateway,
            503 => Self::ServiceUnavailable,
            504 => Self::GatewayTimeout,
            505 => Self::HTTPVersionNotSupported,
            506 => Self::VariantAlsoNegotiates,
            507 => Self::InsufficientStorage,
            508 => Self::LoopDetected,
            509 => Self::BandwidthLimitExceeded,
            510 => Self::NotExtended,
            511 => Self::NetworkAuthenticationRequired,

            _ => panic!("[UNREACHABLE] Received unknown HTTP code: {code}"),
        }
    }
}

impl serde::ser::Serialize for ResponseStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            ResponseStatus::Continue => {
                serializer.serialize_unit_variant("ResponseStatus", 0, "INFORMATION")
            }
            ResponseStatus::SwitchingProtocols => {
                serializer.serialize_unit_variant("ResponseStatus", 1, "SWITCHING_PROTOCOLS")
            }
            ResponseStatus::Processing => {
                serializer.serialize_unit_variant("ResponseStatus", 2, "PROCESSING")
            }
            ResponseStatus::EarlyHints => {
                serializer.serialize_unit_variant("ResponseStatus", 3, "EARLY_HINTS")
            }
            ResponseStatus::Ok => serializer.serialize_unit_variant("ResponseStatus", 4, "OK"),
            ResponseStatus::Created => {
                serializer.serialize_unit_variant("ResponseStatus", 5, "CREATED")
            }
            ResponseStatus::Accepted => {
                serializer.serialize_unit_variant("ResponseStatus", 6, "ACCEPTED")
            }
            ResponseStatus::NonAuthoritativeInformation => serializer.serialize_unit_variant(
                "ResponseStatus",
                7,
                "NON_AUTHORITATIVE_INFORMATION",
            ),
            ResponseStatus::NoContent => {
                serializer.serialize_unit_variant("ResponseStatus", 8, "NO_CONTENT")
            }
            ResponseStatus::ResetContent => {
                serializer.serialize_unit_variant("ResponseStatus", 9, "RESET_CONTENT")
            }
            ResponseStatus::PartialContent => {
                serializer.serialize_unit_variant("ResponseStatus", 10, "PARTIAL_CONTENT")
            }
            ResponseStatus::MultiStatus => {
                serializer.serialize_unit_variant("ResponseStatus", 11, "MULTI_STATUS")
            }
            ResponseStatus::AlreadyReported => {
                serializer.serialize_unit_variant("ResponseStatus", 12, "ALREADY_REPORTED")
            }
            ResponseStatus::IMUsed => {
                serializer.serialize_unit_variant("ResponseStatus", 13, "IM_USED")
            }
            ResponseStatus::MultipleChoices => {
                serializer.serialize_unit_variant("ResponseStatus", 14, "MULTIPLE_CHOICES")
            }
            ResponseStatus::MovedPermanently => {
                serializer.serialize_unit_variant("ResponseStatus", 15, "MOVED_PERMANENTLY")
            }
            ResponseStatus::Found => {
                serializer.serialize_unit_variant("ResponseStatus", 16, "FOUND")
            }
            ResponseStatus::SeeOther => {
                serializer.serialize_unit_variant("ResponseStatus", 17, "SEE_OTHER")
            }
            ResponseStatus::NotModified => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "NOT_MODIFIED")
            }
            ResponseStatus::UseProxy => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "USE_PROXY")
            }
            ResponseStatus::TemporaryRedirect => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "TEMPORARY_REDIRECT")
            }
            ResponseStatus::PermanentRedirect => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "PERMANENT_REDIRECT")
            }
            ResponseStatus::BadRequest => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "BAD_REQUEST")
            }
            ResponseStatus::Unauthorized => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "UNAUTHORIZED")
            }
            ResponseStatus::PaymentRequired => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "PAYMENT_REQUIRED")
            }
            ResponseStatus::Forbidden => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "FORBIDDEN")
            }
            ResponseStatus::NotFound => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "NOT_FOUND")
            }
            ResponseStatus::MethodNotAllowed => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "METHOD_NOT_ALLOWED")
            }
            ResponseStatus::NotAcceptable => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "NOT_ACCEPTABLE")
            }
            ResponseStatus::ProxyAuthenticationRequired => serializer.serialize_unit_variant(
                "ResponseStatus",
                18,
                "PROXY_AUTHENTICATION_REQUIRED",
            ),
            ResponseStatus::RequestTimeout => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "REQUEST_TIMEOUT")
            }
            ResponseStatus::Conflict => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "CONFLICT")
            }
            ResponseStatus::Gone => serializer.serialize_unit_variant("ResponseStatus", 18, "GONE"),
            ResponseStatus::LengthRequired => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "LENGTH_REQUIRED")
            }
            ResponseStatus::PreconditionFailed => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "PRECONDITION_FAILED")
            }
            ResponseStatus::PayloadTooLarge => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "PAYLOAD_TOO_LARGE")
            }
            ResponseStatus::URITooLong => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "URI_TOO_LONG")
            }
            ResponseStatus::UnsupportedMediaType => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "UNSUPPORTED_MEDIA_TYPE")
            }
            ResponseStatus::RangeNotSatisfiable => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "RANGE_NOT_SATISFIABLE")
            }
            ResponseStatus::ExpectationFailed => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "EXPECTATION_FAILED")
            }
            ResponseStatus::MisdirectedRequest => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "MISDIRECTED_REQUEST")
            }
            ResponseStatus::UnprocessableEntity => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "UNPROCESSABLE_ENTITY")
            }
            ResponseStatus::Locked => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "LOCKED")
            }
            ResponseStatus::FailedDependency => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "FAILED_DEPENDENCY")
            }
            ResponseStatus::TooEarly => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "TOO_EARLY")
            }
            ResponseStatus::UpgradeRequired => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "UPGRADE_REQUIRED")
            }
            ResponseStatus::PreconditionRequired => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "PRECONDITION_REQUIRED")
            }
            ResponseStatus::TooManyRequests => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "TOO_MANY_REQUESTS")
            }
            ResponseStatus::RequestHeaderFieldsTooLarge => serializer.serialize_unit_variant(
                "ResponseStatus",
                18,
                "REQUEST_HEADER_FIELDS_TOO_LARGE",
            ),
            ResponseStatus::UnavailableForLegalReasons => serializer.serialize_unit_variant(
                "ResponseStatus",
                18,
                "UNAVAILABLE_FOR_LEGAL_REASONS",
            ),
            ResponseStatus::InternalServerError => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "INTERNAL_SERVER_ERROR")
            }
            ResponseStatus::NotImplemented => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "NOT_IMPLEMENTED")
            }
            ResponseStatus::BadGateway => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "BAD_GATEWAY")
            }
            ResponseStatus::ServiceUnavailable => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "SERVICE_UNAVAILABLE")
            }
            ResponseStatus::GatewayTimeout => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "GATEWAY_TIMEOUT")
            }
            ResponseStatus::HTTPVersionNotSupported => serializer.serialize_unit_variant(
                "ResponseStatus",
                18,
                "HTTP_VERSION_NOT_SUPPORTED",
            ),
            ResponseStatus::VariantAlsoNegotiates => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "VARIANT_ALSO_NEGOTIATES")
            }
            ResponseStatus::InsufficientStorage => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "INSUFFICIENT_STORAGE")
            }
            ResponseStatus::LoopDetected => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "LOOP_DETECTED")
            }
            ResponseStatus::BandwidthLimitExceeded => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "BANDWIDTH_LIMIT_EXCEEDED")
            }
            ResponseStatus::NotExtended => {
                serializer.serialize_unit_variant("ResponseStatus", 18, "NOT_EXTENDED")
            }
            ResponseStatus::NetworkAuthenticationRequired => serializer.serialize_unit_variant(
                "ResponseStatus",
                18,
                "NETWORK_AUTHENTICATION_REQUIRED",
            ),
        }
    }
}
