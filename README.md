
# Exploring Kotlin and Node.js Microservice application with gRPC

This project is separated into two parts:

- Server: A Kotlin app where gRPC serves the remote calls defined in the proto file
- Client: Express/Node/Bootstrap web page to CRUD the server operations.

To run this app locally, run the following commands in separate command line windows:

- Inside the /server folder: `./gradlew bootRun`
- Inside the /client folder: `npm install && node src/index`

Go to http://localhost:3000/ and try the _Mountains_ application. The client application communicates with the Kotlin server through port 8321 via gRPC.

## Running on Kubernetes

This project includes Dockerfiles to package the client and server into container images. In turn, there is a Katacoda scenario called [Kotlin to Kubernetes](https://katacoda.com/javajon/courses/kubernetes-containers) that shows how this application runs on Kubernetes.

## References

This tutorial was adapted from [LogRocket's article](https://blog.logrocket.com/creating-a-crud-api-with-node-express-and-grpc/).
