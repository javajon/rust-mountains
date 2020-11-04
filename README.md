# Exploring Node.js Microservice application with gRPC

This project is separated into two parts:

- Server: where gRPC serves the remote calls defined in the proto file
- Client: Express/Node/Bootstrap web page to CRUD the server operations.

To run this app locally, run the following commands in separate command line windows:

- Inside the /server folder: `cargo run --bin mountains_server`
- Inside the /client folder: `npm install && node index`

Go to http://localhost:3000/ and try the _Mountains_ application.

## Running on Kubernetes

This project includes Dockerfiles to package the client and server into container images. In turn, there is a Katacoda scenario called [Node.js to Kubernetes](https://katacoda.com/javajon/courses/kubernetes-containers) that shows how this application runs on Kubernetes.

## References

This tutorial was adapted from [LogRocket's article](https://blog.logrocket.com/creating-a-crud-api-with-node-express-and-grpc/).


TODO -

Note on brittle version dependencies and cryptic error messages:

With tonic 0.3.1 the best version is works with is "0.2", using latest tokio 0.3.2 will generate errors that will lead you into Rust/Cargo expert land.

Stick with the versions found and tested in: https://github.com/hyperium/tonic

tonic = {version="0.3.1",features = ["tls"]}
tokio = { version = "0.2", features = ["rt-threaded", "time", "stream", "fs", "macros", "uds"] }
