//! gRPC server for the mountains service.
//!
//! Each method maps a gRPC request onto the in-memory model in model.rs and answers
//! with a proper status: NOT_FOUND when an id is unknown rather than a placeholder,
//! so a client can tell the difference between "no such mountain" and "here it is".

use tonic::{Request, Response, Status, transport::Server};

pub mod mountains {
    // The gRPC package name
    tonic::include_proto!("mountains");
}

mod model;
use crate::model::Model;

use mountains::mountain_service_server::{MountainService, MountainServiceServer};
use mountains::{Empty, Mountain, MountainList, MountainRequestId};

#[derive(Default)]
pub struct MyMountains {
    model: Model,
}

#[tonic::async_trait]
impl MountainService for MyMountains {
    async fn get_all(&self, request: Request<Empty>) -> Result<Response<MountainList>, Status> {
        println!("get_all: {:?}", request.remote_addr());
        let mountains = self.model.get_all();
        Ok(Response::new(MountainList { mountains }))
    }

    async fn get(&self, request: Request<MountainRequestId>) -> Result<Response<Mountain>, Status> {
        let id = request.into_inner().id;
        println!("get: {id}");
        self.model
            .get(&id)
            .map(Response::new)
            .ok_or_else(|| Status::not_found(format!("no mountain with id {id}")))
    }

    async fn insert(&self, request: Request<Mountain>) -> Result<Response<Mountain>, Status> {
        let peak = request.into_inner();
        println!("insert: {}", peak.name);
        Ok(Response::new(self.model.insert(peak)))
    }

    async fn update(&self, request: Request<Mountain>) -> Result<Response<Mountain>, Status> {
        let peak = request.into_inner();
        println!("update: {}", peak.id);
        let id = peak.id.clone();
        self.model
            .update(peak)
            .map(Response::new)
            .ok_or_else(|| Status::not_found(format!("no mountain with id {id}")))
    }

    async fn remove(&self, request: Request<MountainRequestId>) -> Result<Response<Empty>, Status> {
        let id = request.into_inner().id;
        println!("remove: {id}");
        if self.model.remove(&id) {
            Ok(Response::new(Empty {}))
        } else {
            Err(Status::not_found(format!("no mountain with id {id}")))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 8321 is the port the client and the Kubernetes Service both expect.
    let addr = "0.0.0.0:8321".parse()?;
    let mountains = MyMountains::default();

    println!("MountainServiceServer listening on {addr}");

    Server::builder()
        .add_service(MountainServiceServer::new(mountains))
        .serve(addr)
        .await?;

    Ok(())
}
