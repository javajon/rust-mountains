# Exploring a Rust Microservice with gRPC

A gRPC service written in Rust, with a Node.js web client, packaged into container images
and deployed to Kubernetes.

This source code is used by the O'Reilly interactive lab
[Kubernetes Containers: Rust to Kubernetes](https://learning.oreilly.com/scenarios/~/9781492090151/),
which builds both container images, pushes them to a private registry, and runs them on a
Kubernetes cluster. The lab runs in a live cluster provisioned for you in the browser, and
is available to O'Reilly Learning subscribers as part of the subscription.

## The two parts

| part | what it is |
| --- | --- |
| `server` | Rust, [tonic](https://github.com/hyperium/tonic) gRPC, serving the contract in `mountains.proto` |
| `client` | Node.js, Express and Handlebars, calling the server over gRPC |

Both speak the same contract, `mountains.proto`, which lives once at the repository root.
Docker's `COPY` cannot reach outside a build context, so both images build **from the
repository root** rather than from their own directory:

```
docker image build -t mountains-client:0.1.0 -f client/Dockerfile .
docker image build -t mountains-server:0.1.0 -f server/Dockerfile .
```

## The gRPC port

The server listens on **8321**. That number appears in three places and they have to
agree: `mountains_server.rs`, the `MOUNTAINS_SERVER` default in `client/src/client.js`,
and the Service and container ports in `kubernetes/mountains-server.yaml`.

## Running locally

Two terminals:

```
cd server && cargo run --bin mountains_server     # gRPC on :8321
cd client && npm ci && npm start                  # web on :3000
```

Then open http://localhost:3000/ and try the Mountains application.

The server needs `protoc` on the build machine: prost 0.14 no longer bundles it.
On Debian or Ubuntu, `apt-get install protobuf-compiler`.

## Versions are pinned on purpose

Every version here is explicit, and the container images are built from pinned bases:

| pin | version |
| --- | --- |
| Rust | 1.97.1 |
| tonic / tonic-prost / prost | 0.14 |
| tokio | 1 |
| cargo-chef | 0.1.78 |
| chisel | v1.4.2 |
| Node | 24 |
| Express | 5 |

`Cargo.lock` and `client/package-lock.json` are **committed**. They used to be in
`.gitignore`, which meant every build resolved dependencies afresh and could break without
a single line of this repository changing - and it did: `cargo install cargo-chef` began
pulling crates that needed a newer compiler than the pinned one.

Bump these deliberately, then re-run the lab's test to confirm nothing broke.

## Deploying

`kubernetes/` holds a Deployment and Service for each part. The image names contain
`$REGISTRY`, which the lab resolves with `envsubst` before applying:

```
envsubst < kubernetes/mountains-server.yaml > mountains-server-resolved.yaml
kubectl apply -f mountains-server-resolved.yaml
```

## References

The client was adapted from
[LogRocket's article](https://blog.logrocket.com/creating-a-crud-api-with-node-express-and-grpc/).
