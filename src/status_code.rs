//! Contains code for using and parsing FTP status codes.

/// Represent a textual interpretation
/// of a particular status code, most were
/// taken from RFC 959.
#[derive(Debug, PartialEq)]
pub enum StatusCodeKind {
    /// Status code 125
    TransferStarted,
    /// Status code 150
    TransferAboutToStart,
    /// Status code 200
    Ok,
    /// Status code 202
    FeatureNotImplemented,
    /// Status code 211,
    SystemStatus,
    /// Status code 214
    HelpMessage,
    /// Status code 215
    NameSystemType,
    /// Status code 220
    ReadyForNewUser,
    /// Status code 221
    ClosingControlConnection,
    /// Status code 226
    RequestActionCompleted,
    /// Status code 230
    UserLoggedIn,
    /// Status code 227
    EnteredPassiveMode,
    /// Status code 229
    EnteredExtendedPassiveMode,
    /// Status code 250
    RequestFileActionCompleted,
    /// Status code 257
    PathCreated,
    /// Status code 331
    PasswordRequired,
    /// Status code 350
    RequestActionPending,
    /// Status code 500
    CommandUnrecognized,
    /// Status code 504
    SecurityMechanismNotImplemented,
    /// Status code 550
    RequestActionDenied,
    /// Status code 553
    FileNameNotAllowed,
    /// Status code not expected by any implementation
    /// on this crate.
    Unknown,
}

impl From<u16> for StatusCodeKind {
    fn from(code: u16) -> StatusCodeKind {
        match code {
            125 => StatusCodeKind::TransferStarted,
            150 => StatusCodeKind::TransferAboutToStart,
            200 => StatusCodeKind::Ok,
            202 => StatusCodeKind::FeatureNotImplemented,
            211 => StatusCodeKind::SystemStatus,
            214 => StatusCodeKind::HelpMessage,
            215 => StatusCodeKind::NameSystemType,
            221 => StatusCodeKind::ClosingControlConnection,
            220 => StatusCodeKind::ReadyForNewUser,
            226 => StatusCodeKind::RequestActionCompleted,
            227 => StatusCodeKind::EnteredPassiveMode,
            229 => StatusCodeKind::EnteredExtendedPassiveMode,
            230 => StatusCodeKind::UserLoggedIn,
            250 => StatusCodeKind::RequestFileActionCompleted,
            257 => StatusCodeKind::PathCreated,
            331 => StatusCodeKind::PasswordRequired,
            350 => StatusCodeKind::RequestActionPending,
            500 => StatusCodeKind::CommandUnrecognized,
            504 => StatusCodeKind::SecurityMechanismNotImplemented,
            550 => StatusCodeKind::RequestActionDenied,
            553 => StatusCodeKind::FileNameNotAllowed,
            _ => StatusCodeKind::Unknown,
        }
    }
}

/// Contains a [StatusCodeKind](enum.StatusCodeKind.html) and
/// the corresponding code for it, this is useful on generating
/// error messages and validity of a particular code.
#[derive(Debug)]
pub struct StatusCode {
    /// The [StatusCodeKind](enum.StatusCodeKind.html)
    pub kind: StatusCodeKind,
    /// The integer representing the status code
    pub code: u16,
}

impl PartialEq for StatusCode {
    fn eq(&self, other: &StatusCode) -> bool {
        self.code == other.code
    }
}

impl StatusCode {
    /// Parse a server response into a [StatusCode](struct.StatusCode.html) struct.
    pub fn parse(text: &str) -> Self {
        let code: &u16 = &text[0..3].parse().unwrap();
        let code: u16 = *code;
        let kind = StatusCodeKind::from(code);

        Self { kind, code }
    }

    /// Returns whether a status code number is valid.
    pub fn is_valid(&self) -> bool {
        self.code > 199 && self.code < 399
    }

    /// Returns whether a status code number is a failure.
    pub fn is_failure(&self) -> bool {
        self.code > 399 && self.code < 599
    }
}
