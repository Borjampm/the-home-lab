use std::time::Duration;
use sysinfo::System;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

pub mod node {
    tonic::include_proto!("node");
}
use node::node_monitor_server::{NodeMonitor, NodeMonitorServer};
use node::{CpuReply, CpuRequest};

#[derive(Default)]
struct Monitor;

#[tonic::async_trait]
impl NodeMonitor for Monitor {
    type StreamCpuStream = ReceiverStream<Result<CpuReply, Status>>;

    async fn stream_cpu(
        &self,
        req: Request<CpuRequest>,
    ) -> Result<Response<Self::StreamCpuStream>, Status> {
        let refresh_ms = req.into_inner().refresh_ms;
        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            let mut system = System::new();
            system.refresh_cpu_usage();
            let cpu_count = system.cpus().len() as u64;
            let mut interval = tokio::time::interval(Duration::from_millis(refresh_ms));
            loop {
                interval.tick().await;
                system.refresh_cpu_usage();
                let total_usage: f32 =
                    system.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>()
                        / cpu_count as f32;
                if tx.send(Ok(CpuReply { cpu_count, total_usage })).await.is_err() {
                    break;
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("NodeMonitor listening on 127.0.0.1:50051");
    Server::builder()
        .add_service(NodeMonitorServer::new(Monitor))
        .serve("127.0.0.1:50051".parse()?)
        .await?;
    Ok(())
}
