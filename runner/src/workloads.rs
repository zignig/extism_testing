use std::collections::HashMap;

// Create and dispatch workloads
// Need to include rcan and storage tricks
use anyhow::Result;
use wash_runtime::{
    types::{Component, LocalResources, Workload, WorkloadStartRequest},
    wit::WitWorld,
};

// This is more making stuff
pub struct Dispatcher {
    name: String,
    world: WitWorld,
}

impl Dispatcher {
    pub fn new(world: WitWorld) -> Self {
        Dispatcher {
            name: "base".to_string(),
            world: world,
        }
    }

    // given a single wasm , Drop it into the testing zone
    fn simple(name: &str, data: Vec<u8>) -> Result<Option<WorkloadStartRequest>> {
        let mut res = LocalResources::default();
        res.config = HashMap::from([("host".to_string(), "localhost:8080".to_string())]);

        // Load the simple component
        let comp = Component {
            name: name.to_string(),
            bytes: data.into(),
            digest: None,
            local_resources: res,
            pool_size: 0,
            ..Default::default()
        };

        let req = WorkloadStartRequest {
            workload_id: uuid::Uuid::new_v4().to_string(),
            workload: Workload {
                namespace: "testing".to_string(),
                name: name.to_string(),
                annotations: HashMap::new(),
                service: None,
                components: vec![comp],
                host_interfaces: vec![],
                volumes: vec![],
            },
        };
        Ok(Some(req))
    }
}

// // Start a workload
// if let Some(path) = args.input {
//     let data = std::fs::read(&path)?;
//     blobs.save(path).await?;
//     let mut res = LocalResources::default();

//     res.config = HashMap::from([("host".to_string(), "localhost:8080".to_string())]);

//     let comp = Component {
//         name: "bob".to_string(),
//         bytes: data.into(),
//         digest: None,
//         local_resources: res,
//         pool_size: 0,
//         ..Default::default()
//     };

//     // let srvs = Service {

//     // }

//     let req = WorkloadStartRequest {
//         workload_id: uuid::Uuid::new_v4().to_string(),
//         workload: Workload {
//             namespace: "test".to_string(),
//             name: "test-workload".to_string(),
//             annotations: HashMap::new(),
//             service: None,
//             components: vec![comp],
//             host_interfaces: r,
//             volumes: vec![],
//         },
//     };
//     let wr = host.workload_start(req).await?;
//     println!("{:#?}", &wr);
// } else {
//     println!("Load and run a default one.");
// }
