# Trying to develop wash-runtime as a crate inside my experiment.

wash-runtime = { git = "https://github.com/wasmCloud/wasmCloud", tag = "v2.11.0", default-features = false, features = [
    "wasi-keyvalue",
    "wasi-config",
    "wasi-logging",
    "wasi-blobstore",
] }
Got lots of red on cargo build , starting with.

       Caused by:
           0: failed to parse dependency directory: \\?\C:\Users\USER\.cargo\git\checkouts\wasmcloud-5015bea8635e07bc\bc5acfb\crates\wash-runtime\wit\deps
           1: failed to parse package: \\?\C:\Users\USER\.cargo\git\checkouts\wasmcloud-5015bea8635e07bc\bc5acfb\crates\wash-runtime\wit\deps\wasmcloud-nats-0.1.2
           2: expected `world`, `interface` or `use`, found '.'
  --> C:\Users\USER\.cargo\git\checkouts\wasmcloud-5015bea8635e07bc\bc5acfb\crates\wash-runtime\src\host\trigger_service\messaging.rs:35:5
   |
35 | /     crate::wasmtime::component::bindgen!({
36 | |         world: "async-messaging",
37 | |         imports: { default: async | trappable | tracing },
38 | |         exports: { default: async | tracing },
39 | |     });
   | |______^
   |
   = note: this error originates in the macro `crate::wasmtime::component::bindgen` (in Nightly builds, run with -Z macro-backtrace for more info)
Turns out that that the wit could not be referenced correctly.

in https://github.com/wasmCloud/wasmCloud/blob/main/crates/wash-runtime/wit/deps/wasmcloud-nats-0.1.3/package.wit the content is

../../../../../wit/nats/wit/world.wit
This path is broken relative to the crates checkout.

I copied the wit file content from the top level into the file.

https://github.com/wasmCloud/wasmCloud/blob/main/wit/nats/wit/world.wit

Now the red in the cargo build are my errors ... ;)