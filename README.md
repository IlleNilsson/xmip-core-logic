# xmip-core-logic

The Logic capability, in one sentence (ADR-0043): a Logic technology turns a
Stream that arrived on a transport into a named operation with typed
arguments, and an operation's result back into a Stream, using a contract to
type both.

SOAP does it with a WSDL, the HTTP API with an OpenAPI document, gRPC with a
protobuf service. None of them moves bytes, that is transport; none of them
orders work, that is process. What they own is the method. Each technology is
its own repository mounted directly under this one: `soap`, `http-api`, `grpc`.

Status: the trait and its shapes are here; three technologies implement it.
