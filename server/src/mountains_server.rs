use std::vec::Vec;
use std::string::ToString;
use tonic::{transport::Server, Request, Response, Status};

pub mod mountains {
    // The gRPC package name
    tonic::include_proto!("mountains");
}

mod model;
use crate::model::*;

use mountains::mountain_service_server::{MountainService, MountainServiceServer};
use mountains::{Empty, Mountain, MountainList, MountainRequestId};

#[derive(Default)]
pub struct MyMountains {
    mountains_model: Model
}

#[tonic::async_trait]
impl MountainService for MyMountains {
    async fn get_all(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<MountainList>, Status> {
        println!("Got a request: {:#?}", &request);
        
        // let mut peaks = vec![];
        // for i in self.mountains_model.get_all() {
        //     let m = Mountain {
        //         id: i.id.clone(),
        //         name: i.name.clone(),
        //         elevation: i.elevation,
        //         location: i.location.clone()    
        //     };
        //     println!("Returning mountain: {:#?}", m.name);
        //     peaks.push(m);
        // }
        // let reply = MountainList { mountains: peaks };

        // Debugging - force in a peak
        let hills = [ 
            Mountain {
                id: "a48a8cce-fba6-11ea-adc1-0242ac120002".to_string(),
                name: "Flatirons".to_string(),
                elevation: 2484,
                location: "39.988_N_105.293_W".to_string()
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

        let mountain = Mountain {
            id: "1".to_string(),
            name: "todo-name".to_string(),
            elevation: 33,
            location: "todo-here".to_string(),       
        };

        // self.mountains_model.create(mountain);

        Ok(Response::new(mountain))
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
    let addr = "0.0.0.0:8321".parse().unwrap();
    let mountains = MyMountains::default();

    println!("MountainServiceServer listening on {}", addr);   

    Server::builder()
        .add_service(MountainServiceServer::new(mountains))
        .serve(addr)
        .await?;

    Ok(())
}
