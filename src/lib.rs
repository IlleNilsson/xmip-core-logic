#![forbid(unsafe_code)]
//! Common contracts for method and operation semantics.
//!
//! This planned capability sits above Transport, Message, and Contract. It
//! does not own byte movement, representation, schema evaluation, or
//! Receive/Send orchestration. No public Logic API is implemented yet.
