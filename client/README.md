# Client

## Run Server

```sh
social (main) [SIGINT]> cargo run --bin social
   Compiling social v0.1.0 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.73s
     Running `target/debug/social`
starting social service server
init social service server...
serve social service at [::1]:9090
get request: Request { metadata: MetadataMap { headers: {"te": "trailers", "content-type": "application/grpc", "user-agent": "tonic/0.11.0"} }, message: GreetRequest { msg: "John" }, extensions: Extensions }
```

## Run Client

```sh
social (main)> cargo run --bin social-client
   Compiling social v0.1.0 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.18s
     Running `target/debug/social-client`
starting social service client
call greet with request: Request { metadata: MetadataMap { headers: {} }, message: GreetRequest { msg: "John" }, extensions: Extensions }
get greet_resp from greet: GreetResponse { msg: "Hello John, Welcome!" }
```
