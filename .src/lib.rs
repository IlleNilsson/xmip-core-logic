#![forbid(unsafe_code)]

//! The Logic capability, in one sentence (ADR-0043, resolving open problem 6):
//!
//! **A Logic technology turns a Stream that arrived on a transport into a
//! named operation with typed arguments, and an operation's result back into
//! a Stream, using a contract to type both.**
//!
//! SOAP does it with a WSDL, the HTTP API with an `OpenAPI` document, gRPC with
//! a protobuf service. None of them moves bytes — that is transport — and none
//! of them orders work — that is process. What they own is the *method*: which
//! operation a Stream is asking for, what its arguments are, and how an answer
//! or a fault travels back on the same reply channel.
//!
//! Both directions, one technology (ADR-0010's direction neutrality, applied to
//! methods): on a Receive Location a technology reads an [`Arrival`] into an
//! [`Invocation`] and writes an [`Outcome`] into a reply Stream; on a Send
//! Location it writes an [`Invocation`] into a [`Request`] and reads the reply
//! Stream into an [`Outcome`]. The transport carries the bytes either way.

use contract::ContractId;
use stream::Stream;

/// An operation's name in its service's own terms: `OrderService` and
/// `PlaceOrder` for SOAP, an `operationId` for an HTTP API, `package.Service`
/// and `Method` for gRPC.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OperationName {
    pub service: String,
    pub name: String,
}

impl OperationName {
    pub fn new(service: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            service: service.into(),
            name: name.into(),
        }
    }
}

/// A name and value pair as transports carry them beside a body: an HTTP
/// header, a gRPC metadata entry, a SOAP action.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Header {
    pub name: String,
    pub value: String,
}

impl Header {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

/// What a transport delivered: where it was addressed, how, what came beside
/// the body, and the body itself. The technology reads the operation out of
/// these; it never sees the socket.
pub struct Arrival<'a> {
    /// The path, action or channel the caller addressed.
    pub target: &'a str,
    /// The method or verb, where the transport has one; empty otherwise.
    pub method: &'a str,
    pub headers: &'a [Header],
    pub body: &'a Stream,
}

/// A named operation with its arguments. The arguments are a Stream in the
/// representation the technology implies — XML for SOAP, JSON for an HTTP API,
/// protobuf for gRPC — and the contract that types them, when the technology
/// can name one.
#[derive(Clone, Debug)]
pub struct Invocation {
    pub operation: OperationName,
    pub arguments: Stream,
    /// Values that arrived beside the body and belong to the operation: path
    /// and query parameters, a correlation id, a message id.
    pub parameters: Vec<Header>,
    pub contract: Option<ContractId>,
}

/// Why an operation did not produce a result: the service's own fault, in the
/// technology's vocabulary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fault {
    /// The code the technology puts on the wire: a SOAP fault code, an HTTP
    /// status, a gRPC status.
    pub code: String,
    pub message: String,
}

/// What an operation came back with.
#[derive(Clone, Debug)]
pub enum Outcome {
    Result(Stream),
    Fault(Fault),
}

/// What a transport should send: the same four things an [`Arrival`] carries,
/// owned, for the Send side.
#[derive(Clone, Debug)]
pub struct Request {
    pub target: String,
    pub method: String,
    pub headers: Vec<Header>,
    pub body: Stream,
}

/// A reply the transport should carry back, or received: the body and what
/// travels beside it, an HTTP status or gRPC trailers among them.
#[derive(Clone, Debug)]
pub struct Reply {
    pub headers: Vec<Header>,
    pub body: Stream,
}

#[derive(Debug)]
pub struct LogicError {
    pub message: String,
}

impl LogicError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl core::fmt::Display for LogicError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for LogicError {}

/// The one trait. A technology implements all four directions; a Location
/// uses the two its side needs.
pub trait Logic: Send + Sync {
    /// The token in the repository name: `soap`, `http-api`, `grpc`.
    fn technology(&self) -> &'static str;

    /// Receive side: which operation this arrival asks for, and with what.
    ///
    /// # Errors
    /// The arrival is not this technology's, or names no operation.
    fn invocation(&self, arrival: &Arrival<'_>) -> Result<Invocation, LogicError>;

    /// Receive side: the outcome, in the shape the caller expects back.
    ///
    /// # Errors
    /// The outcome cannot be expressed in this technology.
    fn reply(&self, invocation: &Invocation, outcome: &Outcome) -> Result<Reply, LogicError>;

    /// Send side: the invocation as the transport must send it.
    ///
    /// # Errors
    /// The invocation cannot be expressed in this technology.
    fn request(&self, invocation: &Invocation) -> Result<Request, LogicError>;

    /// Send side: what the service answered.
    ///
    /// # Errors
    /// The reply is not this technology's.
    fn outcome(&self, invocation: &Invocation, reply: &Reply) -> Result<Outcome, LogicError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_name_is_service_and_operation() {
        let name = OperationName::new("OrderService", "PlaceOrder");
        assert_eq!(name.service, "OrderService");
        assert_eq!(name.name, "PlaceOrder");
    }

    #[test]
    fn an_error_displays_its_message() {
        assert_eq!(LogicError::new("no operation").to_string(), "no operation");
    }
}
