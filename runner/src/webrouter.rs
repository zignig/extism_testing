// A custom route for this project

use anyhow::anyhow;
use wash_runtime::{
    engine::workload::ResolvedWorkload, host::{allowed_hosts::AllowedHost, http::{IngressRoute, RouteError, Router}},
};
use wasmtime_wasi_http::{RequestOptions, WasiBody};
use tracing::info;

/// Development router that routes all requests to the last resolved workload
#[derive(Default)]
pub struct EasyRouter {
    last_workload_id: std::sync::RwLock<Option<String>>,
}

#[async_trait::async_trait]
impl Router for EasyRouter {
    async fn on_workload_resolved(
        &self,
        resolved_handle: &ResolvedWorkload,
        component_id: &str,
    ) -> anyhow::Result<()> {
        info!("ATTACH WORKSPACE {:#?} ==> {}",&resolved_handle.namespace(),component_id);
        let mut lock = self
            .last_workload_id
            .write()
            .map_err(|e| anyhow::anyhow!("DevRouter write lock poisoned: {e}"))?;
        lock.replace(resolved_handle.id().to_string());
        Ok(())
    }

    async fn on_workload_unbind(&self, workload_id: &str) -> anyhow::Result<()> {
        let mut lock = self
            .last_workload_id
            .write()
            .map_err(|e| anyhow::anyhow!("DevRouter write lock poisoned: {e}"))?;
        if let Some(current_id) = &*lock
            && current_id == workload_id
        {
            let _ = lock.take();
        }
        Ok(())
    }

    async fn on_service_http_resolved(
        &self,
        workload_id: &str,
        _routes: &[IngressRoute],
    ) -> anyhow::Result<()> {
        // A service-handled workload routes the same way as a component one:
        // DevRouter sends all requests to the most-recently resolved workload,
        // so it ignores hostnames and paths alike.
        let mut lock = self
            .last_workload_id
            .write()
            .map_err(|e| anyhow::anyhow!("DevRouter write lock poisoned: {e}"))?;
        lock.replace(workload_id.to_string());
        Ok(())
    }

    fn allow_outgoing_request(
        &self,
        _workload_id: &str,
        _request: &hyper::Request<WasiBody>,
        _options: Option<RequestOptions>,
        allowed_hosts: &[AllowedHost],
    ) -> anyhow::Result<()> {
        // check_allowed_hosts(request, allowed_hosts)
        Err(anyhow!("borken"))
    }

    /// Pick a workload ID based on the incoming request
    fn route_incoming_request(
        &self,
        req: &hyper::Request<hyper::body::Incoming>,
    ) -> Result<String, RouteError> {
        info!("{:#?}",req.uri());
        let lock = self
            .last_workload_id
            .try_read()
            .map_err(|_| RouteError::Unavailable)?;
        match &*lock {
            Some(id) => Ok(id.clone()),
            // DevRouter is host-agnostic; signal "nothing registered" via an
            // empty host string (see RouteError::NoWorkloadForHost docs).
            None => Err(RouteError::NoWorkloadForHost(String::new())),
        }
    }
}
