# xmip-core-logic

The Logic capability: the method axis. What a Logic technology is, in one
sentence, is ADR-0043, and this file does not quote it: transport moves the
bytes, process orders the work, contract types the content, and Logic owns
which operation a Stream asks for and how its answer travels back.

SOAP does it with a WSDL, the HTTP API with an OpenAPI document, gRPC with a
protobuf service, Matter with an endpoint, cluster and command path in a TLV
invoke. Each technology is its own repository mounted directly under this one:
`soap`, `http-api`, `grpc`, `matter`.

Status: the trait and its shapes are here; four technologies implement it.
`architecture.toml` names them and carries each one's maturity.
