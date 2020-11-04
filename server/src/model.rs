 use mountains::{Mountain};

 pub mod mountains {
    // The gRPC package name
    tonic::include_proto!("mountains");
}

#[derive(Default)]
pub struct Model {
    mountains_journal: Vec<Mountain>
}

impl Model {
    pub fn new() -> Model { Model { mountains_journal: make_mountains() } }

    pub fn get_all(&self) -> Vec<Mountain> {
        let mountains = self.mountains_journal.clone();
        mountains
    }

    // pub fn create(&self, Mountain: peak) {
    //     self.mountains_journal.push(peak);
    // }
}

fn make_mountains() -> Vec<Mountain> { 
    let mountains = vec![ 
        Mountain {
            id: "a48a6488-fba6-11ea-adc1-0242ac120002".to_string(),
            name: "Mt. Washington".to_string(),
            elevation: 1917,
            location: "44_16_13.8_N_71_18_11.7_W".to_string(),
        },
        Mountain {
             id: "a48a8cce-fba6-11ea-adc1-0242ac120002".to_string(),
             name: "Flatirons".to_string(),
             elevation: 2484,
             location: "39.988_N_105.293_W".to_string(),
        }         
    ];

    mountains
}
