use std::collections::HashMap;

// Create and dispatch workloads
// Need to include rcan and storage tricks

use anyhow::Result;
use wash_runtime::{
    types::{Component, LocalResources, Workload, WorkloadStartRequest},
    wit::WitWorld,
};
use bytes::Bytes;


// This is more making stuff
pub struct Dispatcher {
    world: WitWorld,
}

impl Dispatcher {
    pub fn new(world: WitWorld) -> Self {
        Dispatcher {
            world: world,
        }
    }

    // given a single wasm , Drop it into the testing zone
    pub fn simple(&self , name: &str, data: Bytes ) -> Result<WorkloadStartRequest> {
        // just offer up all of the hosts available interface
        let available = self.world.exports.clone().into_iter().collect();

        let res = LocalResources::default();
        
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
                host_interfaces: available,
                volumes: vec![],
            },
        };
        Ok(req)
    }
}