use tonic::{transport::Server, Request, Response, Status};

pub mod mountains {
    // The gRPC package name
    tonic::include_proto!("mountains");
}

use mountains::mountains_service_server::{MountainsService, MountainsServiceServer};
use mountains::{Empty, Mountain, MountainList, MountainRequestId};

#[derive(Default)]
pub struct MyMountains {}

#[tonic::async_trait]
impl MountainsService for MyMountains {
    async fn get_all(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<MountainList>, Status> {
        println!("Got a request: {:#?}", &request);

        let hills = [ 
            Mountain {
                id: "1".to_string(),
                name: "todo-name".to_string(),
                elevation: 33,
                location: "todo-here".to_string(),       
            },
        ];

        let reply = MountainList { mountains: hills.to_vec() };

        Ok(Response::new(reply))
    }

    async fn get(
        &self,
        request: Request<MountainRequestId>,
    ) -> Result<Response<Mountain>, Status> {
        println!("Got a request: {:#?}", &request);

        let reply = Mountain {
            id: "1".to_string(),
            name: "todo-name".to_string(),
            elevation: 33,
            location: "todo-here".to_string(),       
        };

        Ok(Response::new(reply))
    }

    async fn insert(
        &self,
        request: Request<Mountain>,
    ) -> Result<Response<Mountain>, Status> {
        println!("Got a request: {:#?}", &request);

        let reply = Mountain {
            id: "1".to_string(),
            name: "todo-name".to_string(),
            elevation: 33,
            location: "todo-here".to_string(),       
        };

        Ok(Response::new(reply))
    }

    async fn update(
        &self,
        request: Request<Mountain>,
    ) -> Result<Response<Mountain>, Status> {
        println!("Got a request: {:#?}", &request);

        let reply = Mountain {
            id: "1".to_string(),
            name: "todo-name".to_string(),
            elevation: 33,
            location: "todo-here".to_string(),       
        };

        Ok(Response::new(reply))
    }

    async fn remove(
        &self,
        request: Request<MountainRequestId>,
    ) -> Result<Response<Empty>, Status> {
        println!("Got a request: {:#?}", &request);

        let reply = Empty {};

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:3000".parse().unwrap();
    let mountains = MyMountains::default();

    println!("MountainsServiceServer listening on {}", addr);

    Server::builder()
        .add_service(MountainsServiceServer::new(mountains))
        .serve(addr)
        .await?;

    Ok(())
}
