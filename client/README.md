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

## Run Service with Stream

```sh
2024-09-16T14:30:46.684960Z  INFO social: starting social service server
2024-09-16T14:30:46.685172Z  INFO social: init social service server...
2024-09-16T14:30:46.685248Z  INFO social: serve social service at [::1]:9090
get request: Request { metadata: MetadataMap { headers: {"te": "trailers", "content-type": "application/grpc", "user-agent": "tonic/0.11.0"} }, message: GreetRequest { msg: "John" }, extensions: Extensions }
req: PostFeedRequest { limit: 10 }
send post to sender
send post to sender
send post to sender
2024-09-16T14:31:00.189498Z  WARN social::abi::service: send post failed: SendError { .. }
send post to sender
2024-09-16T14:31:03.191782Z  WARN social::abi::service: send post failed: SendError { .. }
send post to sender
2024-09-16T14:31:06.194603Z  WARN social::abi::service: send post failed: SendError { .. }
```

## Run Client with Stream

```sh
starting social service client
call greet with request: Request { metadata: MetadataMap { headers: {} }, message: GreetRequest { msg: "John" }, extensions: Extensions }
get greet_resp from greet: GreetResponse { msg: "Hello John, Welcome!" }
call greet with request: Request { metadata: MetadataMap { headers: {} }, message: PostFeedRequest { limit: 10 }, extensions: Extensions }
get posts: [Post { post_id: "a0edad55-8dff-4d8d-8394-6be397481bb7", user: Some(User { user_id: "c52b8a33-82b9-46bc-a648-9420631607c5", nick_name: "Mozelle Lynch", user_name: "Shea Murazik", avatar: "7GW1p", bio: "sdDY14prXdYoQ", email: "noelia@example.com", web_site: "N0Kk00a4uywAGuhn", birthday: "3doDOyyT4", created_at: Some(Timestamp { seconds: 1726497054, nanos: 184573000 }), updated_at: Some(Timestamp { seconds: 1726497054, nanos: 184577000 }) }), content: Some(Content { text: "vMc84inuonKF8rKNw", images: [], videos: [] }), created_at: Some(Timestamp { seconds: 1726497054, nanos: 184598000 }), updated_at: Some(Timestamp { seconds: 1726497054, nanos: 184598000 }) }]
get posts: [Post { post_id: "b5a695fd-109a-464d-a588-739534ca5955", user: Some(User { user_id: "9c16bc55-6f0a-46c3-90e8-c35331685bc5", nick_name: "Jordyn Corkery", user_name: "Guadalupe Roberts", avatar: "tsYLUx6Ulsy", bio: "GVQI5p9Az", email: "gaston@example.org", web_site: "hiCtsRLr", birthday: "E7eaMIUsToyo", created_at: Some(Timestamp { seconds: 1726497057, nanos: 187762000 }), updated_at: Some(Timestamp { seconds: 1726497057, nanos: 187771000 }) }), content: Some(Content { text: "OHudYof2CC9rbISu", images: [], videos: [] }), created_at: Some(Timestamp { seconds: 1726497057, nanos: 187831000 }), updated_at: Some(Timestamp { seconds: 1726497057, nanos: 187832000 }) }]
```